#[derive(PartialEq, Debug)]
pub enum Json { Bool(bool), Int(i64), Float(f64), String(String), Array(Vec<Json>), Dict(Vec<(String, Json)>), Literal(String) }
impl Json {

	/// Create a new JSON struct from JSON contents.
	pub fn new(contents:&str) -> Option<Json> {
		JsonParser::new(contents).parse_all()
	}
}



struct Tag {
	str:&'static str,
	len:usize
}
impl Tag {
	const fn new(str:&'static str) -> Tag {
		Tag {
			str,
			len: str.len()
		}
	}
}



pub struct JsonParser<'a> {
	contents:&'a str,
	contents_len:usize,
	cursor:usize
}
impl<'a> JsonParser<'a> {

	/* CONSTRUCTOR METHODS */

	/// Create a new JSON parser.
	fn new(contents:&'a str) -> JsonParser<'a> {
		JsonParser {
			contents,
			contents_len: contents.len(),
			cursor: 0
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
			let type_parser_methods:&[fn(&mut JsonParser<'a>) -> Option<Json>] = &[Self::parse_bool, Self::parse_number, Self::parse_string, Self::parse_array];
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
		const TRUE_TAG:Tag = Tag::new("true");
		const FALSE_TAG:Tag = Tag::new("false");
		const FLIP_TAG:Tag = Tag::new("!");
		let remainder:&str = self.remainder();
		let remainder_len:usize = remainder.len();

		// True.
		if remainder_len >= TRUE_TAG.len && remainder[..TRUE_TAG.len].to_lowercase() == TRUE_TAG.str {
			self.cursor += TRUE_TAG.len;
			return Some(Json::Bool(true));
		}

		// False.
		if remainder_len >= FALSE_TAG.len && remainder[..FALSE_TAG.len].to_lowercase() == FALSE_TAG.str {
			self.cursor += FALSE_TAG.len;
			return Some(Json::Bool(false));
		}

		// Flip.
		if remainder_len >= FLIP_TAG.len && remainder[..FLIP_TAG.len].to_lowercase() == FLIP_TAG.str {
			let cursor_origin:usize = self.cursor;
			self.cursor += 1;
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
		const FLOAT_SEPARATOR_TAG:Tag = Tag::new(".");
		const NEGATIVE_TAG:Tag = Tag::new("-");
		const MARKUP_TAGS:&[Tag] = &[Tag::new("_")];

		/// Wether or not the given str is considered numeric.
		fn str_is_number_like(str:&str) -> bool {
			str == FLOAT_SEPARATOR_TAG.str ||
			MARKUP_TAGS.iter().any(|tag| tag.str == str) ||
			str.chars().all(|char| char.is_numeric())
		}

		self.try_with_cursor(|inner_self| {

			// Negative flipping.
			let mut flip_negative:bool = false;
			while inner_self.skip_if_remainder_starts_with(NEGATIVE_TAG.str) {
				flip_negative = !flip_negative;
			}

			// Collect number.
			let number_start:usize = inner_self.cursor;
			while inner_self.next_char_str().is_some_and(str_is_number_like) {
				inner_self.cursor += 1;
			}
			let mut number_str:String = inner_self.contents[number_start..inner_self.cursor].to_string();
			for tag in MARKUP_TAGS {
				number_str = number_str.replace(tag.str, "");
			}

			// Return JSON.
			if !number_str.chars().any(|char| char.is_numeric()) {
				None
			} else {
				Some(
					if number_str.contains(FLOAT_SEPARATOR_TAG.str) {
						Json::Float(number_str.parse::<f64>().unwrap() * if flip_negative { -1.0 } else { 1.0 })
					} else {
						Json::Int(number_str.parse::<i64>().unwrap() * if flip_negative { -1 } else { 1 })
					}
				)
			}
		})
	}

	/// Try to parse a string.
	/// Increments the cursor when any match is returned.
	/// Includes quotation marks.
	fn parse_string(&mut self) -> Option<Json> {
		const STRING_MATCHERS:&[(Tag, Tag, &[(Tag, usize)])] = &[
			(Tag::new("'"), Tag::new("'"), &[(Tag::new("\\"), 1)]),
			(Tag::new("\""), Tag::new("\""), &[(Tag::new("\\"), 1)])
		];

		// Loop through string sets.
		if self.has_remainder() {
			self.try_with_cursor(|inner_self| {
				for (open_tag, close_tag, escapes) in STRING_MATCHERS {
					if inner_self.remainder_starts_with(open_tag.str) {
						let string_start:usize = inner_self.cursor;
						inner_self.cursor += open_tag.len;
						while inner_self.has_remainder() {
							if inner_self.skip_if_remainder_starts_with(close_tag.str) {
								return Some(Json::String(inner_self.contents[string_start..inner_self.cursor].to_string()));
							}
							for (escape_tag, escape_skip) in *escapes {
								if inner_self.skip_if_remainder_starts_with(escape_tag.str) {
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
		const ARRAY_START_TAG:Tag = Tag::new("[");
		const ARRAY_END_TAG:Tag = Tag::new("]");
		const ARRAY_SEPARATOR_TAG:Tag = Tag::new(",");

		self.try_with_cursor(|inner_self| {
			if inner_self.skip_if_remainder_starts_with(ARRAY_START_TAG.str) {
				let mut sub_json:Vec<Json> = Vec::new();
				loop {
					inner_self.skip_whitespace();
					let remainder:&str = inner_self.remainder();
					if remainder.starts_with(ARRAY_END_TAG.str) {
						inner_self.cursor += ARRAY_END_TAG.len;
						break;
					} else if remainder.starts_with(ARRAY_SEPARATOR_TAG.str) {
						inner_self.cursor += ARRAY_SEPARATOR_TAG.len;
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