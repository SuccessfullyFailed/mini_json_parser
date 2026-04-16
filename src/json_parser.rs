use std::{ error::Error, sync::Arc };
use file_ref::FileRef;



#[derive(PartialEq, Debug)]
pub enum Json { Bool(bool), Int(i64), Float(f64), String(String), Array(Vec<Json>), Dict(Vec<(Json, Option<Json>)>) }
impl Json {

	/// Create a new JSON struct from a file.
	pub fn from_file(file_path:&str) -> Result<Json, Box<dyn Error>> {
		Json::from_file_with_tag_set(file_path, JsonTagSet::default())
	}

	/// Create a new JSON struct from a file with the specified tag set.
	pub fn from_file_with_tag_set(file_path:&str, tag_set:JsonTagSet) -> Result<Json, Box<dyn Error>> {
		let file_contents:String = FileRef::new(file_path).read()?;
		match Json::new_with_tag_set(&file_contents, tag_set) {
			Some(json) => Ok(json),
			None => Err("Could not parse file contents into json.".into())
		}
	}

	/// Create a new JSON struct from JSON contents.
	pub fn new(contents:&str) -> Option<Json> {
		JsonParser::new(contents, JsonTagSet::default()).parse_all()
	}

	/// Create a new JSON struct from JSON contents with the specified tag set.
	pub fn new_with_tag_set(contents:&str, tag_set:JsonTagSet) -> Option<Json> {
		JsonParser::new(contents, tag_set).parse_all()
	}
}
impl ToString for Json {
	fn to_string(&self) -> String {
		match self {
			Json::Bool(value) => (if *value { "true" } else { "false" }).to_string(),
			Json::Int(value) => value.to_string(),
			Json::Float(value) => value.to_string(),
			Json::String(value) => {
				if value.contains("\"") {
					format!("'{}'", value)
				} else {
					format!("\"{}\"", value)
				}
			},
			Json::Array(sub_json) => format!("[{}]", sub_json.iter().map(|json| json.to_string()).collect::<Vec<String>>().join(",")),
			Json::Dict(sub_json) => format!("{{{}}}", sub_json.iter().map(|(key, value)| key.to_string() + &value.as_ref().map(|v| ":".to_string() + &v.to_string()).unwrap_or_default()).collect::<Vec<String>>().join(","))
		}
	}
}



pub struct JsonTagSet {
	bool_true:String,
	bool_false:String,
	bool_flip:String,

	number_decimal_separator:String,
	number_negative:String,
	number_decoration:Vec<String>,

	string_quotes_set:Vec<(String, String, Vec<(String, usize)>)>, // Open, close [escape tag, skip size]

	array_open:String,
	array_separator:String,
	array_close:String,

	dict_open:String,
	dict_value_separator:String,
	dict_entry_separator:String,
	dict_close:String
}
impl JsonTagSet {

	/// Create a new tag set.
	pub fn new(bool_true:&str, bool_false:&str, bool_flip:&str, number_decimal_separator:&str, number_negative:&str, number_decoration:&[&str], string_quotes_set:&[(&str, &str, &[(&str, usize)])], array_open:&str, array_separator:&str, array_close:&str, dict_open:&str, dict_value_separator:&str, dict_entry_separator:&str, dict_close:&str) -> JsonTagSet {
		JsonTagSet {
			bool_true: bool_true.to_string(),
			bool_false: bool_false.to_string(),
			bool_flip: bool_flip.to_string(),

			number_decimal_separator: number_decimal_separator.to_string(),
			number_negative: number_negative.to_string(),
			number_decoration: number_decoration.iter().map(|str| str.to_string()).collect(),

			string_quotes_set: {
				string_quotes_set.iter().map(|(open, close, escapes)| (
					open.to_string(),
					close.to_string(),
					escapes.iter().map(|(escape_tag, escape_size)|
						(escape_tag.to_string(), *escape_size)
					).collect::<Vec<(String, usize)>>()
				)).collect()
			},

			array_open: array_open.to_string(),
			array_close: array_close.to_string(),
			array_separator: array_separator.to_string(),

			dict_open: dict_open.to_string(),
			dict_value_separator: dict_value_separator.to_string(),
			dict_entry_separator: dict_entry_separator.to_string(),
			dict_close: dict_close.to_string()
		}
	}
}
impl Default for JsonTagSet {
	fn default() -> Self {
		JsonTagSet::new(
			"true", "false", "!",
			".", "-", &["_"],
			&[("'", "'", &[("\\", 1)]), ("\"", "\"", &[("\\", 1)])],
			"[", ",", "]",
			"{", ":", ",", "}"
		)
	}
}



pub struct JsonParser<'a> {
	contents:&'a str,
	contents_len:usize,
	cursor:usize,
	tag_set:Arc<JsonTagSet>
}
impl<'a> JsonParser<'a> {

	/* CONSTRUCTOR METHODS */

	/// Create a new JSON parser.
	fn new(contents:&'a str, tag_set:JsonTagSet) -> JsonParser<'a> {
		JsonParser {
			contents,
			contents_len: contents.len(),
			cursor: 0,
			tag_set: Arc::new(tag_set)
		}
	}



	/* PARSING METHODS */

	/// Parse all contents.
	fn parse_all(&mut self) -> Option<Json> {
		self.parse_next()
	}

	/// Parse the next-first thing.
	fn parse_next(&mut self) -> Option<Json> {

		// Skip whitespace and make sure more contents exist.
		self.skip_whitespace();
		if !self.has_remainder() {
			return None;
		}

		// Try any of the available parser methods.
		if self.cursor < self.contents_len {
			let type_parser_methods:&[fn(&mut JsonParser<'a>) -> Option<Json>] = &[Self::parse_bool, Self::parse_number, Self::parse_string, Self::parse_array, Self::parse_dict];
			for parse_method in type_parser_methods {
				if let Some(json) = parse_method(self) {
					return Some(json);
				}
			}
		}

		// No json was found, return nothing.
		None
	}

	/// Try to parse a boolean.
	/// Recognizes 'True' and 'False', with our without capital characters.
	fn parse_bool(&mut self) -> Option<Json> {
		let remainder:&str = self.remainder();
		let remainder_len:usize = remainder.len();

		// True.
		let true_tag:&str = &self.tag_set.bool_true;
		let true_tag_len:usize = true_tag.len();
		if remainder_len >= true_tag_len && remainder[..true_tag_len].to_lowercase() == true_tag {
			self.cursor += true_tag_len;
			return Some(Json::Bool(true));
		}

		// False.
		let false_tag:&str = &self.tag_set.bool_false;
		let false_tag_len:usize = false_tag.len();
		if remainder_len >= false_tag_len && remainder[..false_tag_len].to_lowercase() == false_tag {
			self.cursor += false_tag_len;
			return Some(Json::Bool(false));
		}

		// Flip.
		let flip_tag:&str = &self.tag_set.bool_flip;
		let flip_tag_len:usize = flip_tag.len();
		if remainder_len >= flip_tag_len && remainder[..flip_tag_len].to_lowercase() == flip_tag {
			let cursor_origin:usize = self.cursor;
			self.cursor += flip_tag_len;
			if let Some(sub_bool) = self.parse_bool() {
				if let Json::Bool(bool) = sub_bool {
					return Some(Json::Bool(!bool));
				}
			}
			self.cursor = cursor_origin;
		}
		
		// No boolean found.
		None
	}

	/// Try to parse a number.
	/// Can return an integer or a float.
	fn parse_number(&mut self) -> Option<Json> {

		/// Wether or not the given str is considered numeric.
		fn str_is_number_like(tag_set:&JsonTagSet, str:&str) -> bool {
			str == tag_set.number_decimal_separator ||
			tag_set.number_decoration.iter().any(|tag| *tag == str) ||
			str.chars().all(|char| char.is_numeric())
		}

		self.try_with_cursor(|inner_self| {
			let tags:Arc<JsonTagSet> = Arc::clone(&inner_self.tag_set);

			// Negative flipping.
			let mut flip_negative:bool = false;
			while inner_self.skip_if_remainder_starts_with(&tags.number_negative) {
				flip_negative = !flip_negative;
			}

			// Collect number.
			let number_start:usize = inner_self.cursor;
			while inner_self.next_char_str().is_some_and(|str| str_is_number_like(&tags, str)) {
				inner_self.cursor += 1;
			}
			let mut number_str:String = inner_self.contents[number_start..inner_self.cursor].to_string();
			for tag in &tags.number_decoration {
				number_str = number_str.replace(tag, "");
			}

			// Return JSON.
			if !number_str.chars().any(|char| char.is_numeric()) {
				None
			} else {
				Some(
					if number_str.contains(&tags.number_decimal_separator) {
						Json::Float(number_str.replace(&tags.number_decimal_separator, ".").parse::<f64>().unwrap() * if flip_negative { -1.0 } else { 1.0 })
					} else {
						Json::Int(number_str.parse::<i64>().unwrap() * if flip_negative { -1 } else { 1 })
					}
				)
			}
		})
	}

	/// Try to parse a string.
	/// Increments the cursor when any match is returned.
	/// Excludes quotation marks.
	fn parse_string(&mut self) -> Option<Json> {
		if self.has_remainder() {
			self.try_with_cursor(|inner_self| {
				let tags:Arc<JsonTagSet> = Arc::clone(&inner_self.tag_set);
				for (open_tag, close_tag, escapes) in &tags.string_quotes_set {
					if inner_self.remainder_starts_with(open_tag) {
						inner_self.cursor += open_tag.len();
						let string_start:usize = inner_self.cursor;
						while inner_self.has_remainder() {
							if inner_self.skip_if_remainder_starts_with(close_tag) {
								return Some(Json::String(inner_self.contents[string_start..inner_self.cursor - close_tag.len()].to_string()));
							}
							for (escape_tag, escape_skip) in escapes {
								if inner_self.skip_if_remainder_starts_with(escape_tag) {
									inner_self.cursor += escape_skip - 1; // Remove one for the cursor incrementation at the end of the loop.
								}
							}
							inner_self.cursor += 1;
						}
					}
				}
				None
			})
		} else {
			None
		}
	}

	/// Try to parse an array.
	/// Increments the cursor when any match is returned.
	fn parse_array(&mut self) -> Option<Json> {
		self.try_with_cursor(|inner_self| {
			let tags:Arc<JsonTagSet> = Arc::clone(&inner_self.tag_set);
			if inner_self.skip_if_remainder_starts_with(&tags.array_open) {
				let mut sub_json:Vec<Json> = Vec::new();
				loop {
					inner_self.skip_whitespace();
					let remainder:&str = inner_self.remainder();
					if remainder.starts_with(&tags.array_close) {
						inner_self.cursor += tags.array_close.len();
						break;
					} else if remainder.starts_with(&tags.array_separator) {
						inner_self.cursor += tags.array_separator.len();
						continue;
					} else if let Some(sub_match) = inner_self.parse_next() {
						sub_json.push(sub_match);
					} else {
						return None;
					}
				}
				Some(Json::Array(sub_json))
			} else {
				None
			}
		})
	}

	/// Try to parse a dictionary.
	/// Increments the cursor when any match is returned.
	fn parse_dict(&mut self) -> Option<Json> {

		// Match dict start.
		self.try_with_cursor(|inner_self| {
			let tags:Arc<JsonTagSet> = Arc::clone(&inner_self.tag_set);
			if inner_self.skip_if_remainder_starts_with(&tags.dict_open) {
				inner_self.skip_whitespace();

				// Keep collection json keys.
				let mut sub_json:Vec<(Json, Option<Json>)> = Vec::new();
				while let Some(key) = inner_self.parse_next() {
					inner_self.skip_whitespace();

					// Match value to the key.
					let value:Option<Json> = {
						if inner_self.skip_if_remainder_starts_with(&tags.dict_value_separator) {
							inner_self.parse_next()
						} else {
							None
						}
					};
					sub_json.push((key, value));
					inner_self.skip_whitespace();

					// Continue matching more values, end the dict or conclude invalid dict.
					if inner_self.skip_if_remainder_starts_with(&tags.dict_entry_separator) {
						inner_self.skip_whitespace();
					} else if inner_self.skip_if_remainder_starts_with(&tags.dict_close) {
						break;
					} else {
						return None;
					}
				}
				Some(Json::Dict(sub_json))
			} else {
				None
			}
		})
	}
	


	/* HELPER METHODS */

	/// Wether or not there are any remaining contents.
	fn has_remainder(&self) -> bool {
		self.cursor < self.contents_len
	}

	/// Get the remainder of the contents.
	fn remainder(&self) -> &str {
		if self.cursor < self.contents_len {
			&self.contents[self.cursor..]
		} else {
			""
		}
	}

	/// Wether or not the remainders starts with the given str.
	fn remainder_starts_with(&self, start:&str) -> bool {
		self.remainder().starts_with(start)
	}

	/// If the remainder starts with the given str, move the cursor to after it.
	/// Returns true if the remainder started with the str.
	fn skip_if_remainder_starts_with(&mut self, start:&str) -> bool {
		let starts_with:bool = self.remainder_starts_with(start);
		if starts_with {
			self.cursor += start.len();
		}
		starts_with
	}

	/// Execute the action.
	/// If the action does not return JSON, reset the cursor to what it was before the action.
	fn try_with_cursor<Action:FnOnce(&mut Self) -> Option<Json>>(&mut self, action:Action) -> Option<Json> {
		let cursor_backup:usize = self.cursor;
		let result:Option<Json> = action(self);
		if result.is_none() {
			self.cursor = cursor_backup;
		}
		result
	}

	/// Get the next character as a str.
	fn next_char_str(&self) -> Option<&str> {
		if self.cursor < self.contents_len {
			Some(&self.contents[self.cursor..self.cursor + 1])
		} else {
			None
		}
	}

	/// Skip whitespace.
	fn skip_whitespace(&mut self) {
		while self.next_char_str().is_some_and(|str| str.chars().next().is_some_and(|char| char.is_whitespace())) {
			self.cursor += 1;
		}
	}
}