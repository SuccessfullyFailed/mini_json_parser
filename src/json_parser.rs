#[derive(PartialEq, Debug)]
pub enum Json { Bool(bool), Int(i64), Float(f64), String(String), Array(Vec<Json>), Dict(Vec<(String, Json)>), Literal(String) }
impl Json {

	/// Create a new JSON struct from JSON contents.
	pub fn new(contents:&str) -> Option<Json> {
		JsonParser::new(contents).parse_all()
	}
}



pub struct JsonParser<'a> {
	contents:&'a str,
	contents_as_chars:Vec<char>,
	contents_len:usize,
	cursor:usize
}
impl<'a> JsonParser<'a> {

	/* CONSTRUCTOR METHODS */

	/// Create a new JSON parser.
	fn new(contents:&'a str) -> JsonParser<'a> {
		JsonParser {
			contents,
			contents_as_chars: contents.chars().collect(),
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

		// Skip whitespace and validate next character.
		self.skip_whitespace();
		if self.cursor >= self.contents.len() {
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
		const TRUE_TAG:&str = "true";
		const TRUE_TAG_LEN:usize = TRUE_TAG.len();
		const FALSE_TAG:&str = "false";
		const FALSE_TAG_LEN:usize = FALSE_TAG.len();
		const FLIP_TAG:&str = "!";
		const FLIP_TAG_LEN:usize = 1;

		if self.cursor < self.contents_len {
			let available:usize = self.contents_len - self.cursor;

			// True.
			if available >= TRUE_TAG_LEN && &self.contents[self.cursor..self.cursor + TRUE_TAG_LEN].to_lowercase() == TRUE_TAG {
				self.cursor += TRUE_TAG_LEN;
				return Some(Json::Bool(true));
			}

			// False.
			if available >= FALSE_TAG_LEN && &self.contents[self.cursor..self.cursor + FALSE_TAG_LEN].to_lowercase() == FALSE_TAG {
				self.cursor += FALSE_TAG_LEN;
				return Some(Json::Bool(false));
			}

			// Flip.
			if available >= FLIP_TAG_LEN && self.contents[self.cursor..self.cursor + FLIP_TAG_LEN].to_lowercase() == FLIP_TAG {
				let cursor_backup:usize = self.cursor;
				self.cursor += 1;
				if let Some(sub_bool) = self.parse_bool() {
					if let Json::Bool(bool) = sub_bool {
						return Some(Json::Bool(!bool));
					}
				}
				self.cursor = cursor_backup;
			}
		}
		None
	}

	/// Try to parse a number.
	/// Can return an integer or a float.
	fn parse_number(&mut self) -> Option<Json> {
		const FLOAT_SEPARATOR_CHAR:char = '.';
		const NEGATIVE:char = '-';
		const SPACING_CHARS:&[char] = &['_'];
		const CHAR_IS_NUMBER_LIKE:fn(char) -> bool = |char| char.is_numeric() || char == FLOAT_SEPARATOR_CHAR || SPACING_CHARS.contains(&char);
		
		if self.cursor < self.contents_len && CHAR_IS_NUMBER_LIKE(self.contents_as_chars[self.cursor]) || self.contents_as_chars[self.cursor] == NEGATIVE {
			let cursor_start:usize = self.cursor;
			self.cursor += 1;
			while self.cursor < self.contents_len && CHAR_IS_NUMBER_LIKE(self.contents_as_chars[self.cursor]) {
				self.cursor += 1;
			}

			let numeric_match:String = self.contents[cursor_start..self.cursor].replace(SPACING_CHARS, "");
			if !numeric_match.chars().any(|char| char.is_numeric()) { // things like '-' or '.'.
				None
			} else if numeric_match.contains(FLOAT_SEPARATOR_CHAR) {
				Some(Json::Float(numeric_match.parse::<f64>().unwrap()))
			} else {
				Some(Json::Int(numeric_match.parse::<i64>().unwrap()))
			}
		} else {
			None
		}
	}

	/// Try to parse a string.
	/// Increments the cursor when any match is returned.
	/// Includes quotation marks.
	fn parse_string(&mut self) -> Option<Json> {
		const STRING_MATCHERS:&[(&str, &str, &[(&str, usize)])] = &[
			("'", "'", &[("\\", 1)]),
			("\"", "\"", &[("\\", 1)])
		];

		// Loop through string sets.
		if self.cursor < self.contents_len {
			let cursor_origin:usize = self.cursor;
			let remainder_len:usize = self.contents_len - self.cursor;
			for &(open_tag, close_tag, escapes) in STRING_MATCHERS {
				let open_tag_len:usize = open_tag.len();
				let close_tag_len:usize = close_tag.len();

				// Try to match open-tag.
				if open_tag_len < remainder_len && &self.contents[..open_tag_len] == open_tag {
					let cursor_max:usize = remainder_len - close_tag_len + 1;
					self.cursor += open_tag_len;
					while self.cursor < cursor_max {

						// Try to match end-tag.
						if &self.contents[self.cursor..self.cursor + close_tag_len] == close_tag {
							self.cursor += close_tag_len;
							return Some(Json::String(self.contents[..self.cursor].to_string()));
						}

						// Try to match escapes.
						for (escape_tag, escape_skip) in escapes {
							let escape_tag_len:usize = escape_tag.len();
							let cursor_escape_end:usize = self.cursor + escape_tag_len;
							if self.cursor < cursor_escape_end && &self.contents[self.cursor..self.cursor + escape_tag_len] == *escape_tag {
								self.cursor += escape_tag_len + escape_skip - 1; // Remove one for the cursor incrementation at the end of the loop.
							}
						}

						// Increment cursor.
						self.cursor += 1;
					}
				}
			}
			self.cursor = cursor_origin;
		}
		
		None
	}

	/// Try to parse an array.
	/// Increments the cursor when any match is returned.
	fn parse_array(&mut self) -> Option<Json> {
		const ARRAY_START_TAG:&str = "[";
		const ARRAY_START_TAG_LEN:usize = ARRAY_START_TAG.len();
		const ARRAY_END_TAG:&str = "]";
		const ARRAY_END_TAG_LEN:usize = ARRAY_END_TAG.len();
		const ARRAY_SEPARATOR_TAG:&str = ",";
		const ARRAY_SEPARATOR_TAG_LEN:usize = ARRAY_SEPARATOR_TAG.len();

		// Match start tag.
		if self.cursor < self.contents_len - ARRAY_START_TAG_LEN && self.contents[self.cursor..].starts_with(ARRAY_START_TAG) {
			let cursor_origin:usize = self.cursor;
			self.cursor += ARRAY_START_TAG_LEN;

			// Sub-match as much as possible.
			let mut sub_json:Vec<Json> = Vec::new();
			let mut parse_invalid:bool = false;
			while let Some(sub_match) = self.parse_next() {
				sub_json.push(sub_match);
				if self.cursor >= self.contents_len {
					parse_invalid = true;
					break;
				} else {
					let remainder:&str = &self.contents[self.cursor..];
					if remainder.starts_with(ARRAY_SEPARATOR_TAG) {
						self.cursor += ARRAY_SEPARATOR_TAG_LEN;
						self.skip_whitespace();
						if self.cursor < self.contents_len && self.contents[self.cursor..].starts_with(ARRAY_END_TAG) {
							break;
						}
					} else if remainder.starts_with(ARRAY_END_TAG) {
						self.cursor += ARRAY_END_TAG_LEN;
						break;
					} else {
						parse_invalid = true;
						break;
					}
				}
			}
			if parse_invalid {
				self.cursor = cursor_origin;
			} else {
				return Some(Json::Array(sub_json));
			}
		}

		// No array was found.
		None
	}


	/* CURSOR METHODS */

	/// Get the next character.
	fn next_char(&self) -> Option<char> {
		if self.cursor < self.contents_as_chars.len() {
			Some(self.contents_as_chars[self.cursor])
		} else {
			None
		}
	}

	/// Skip whitespace.
	fn skip_whitespace(&mut self) {
		while self.next_char().is_some_and(|char| char.is_whitespace()) {
			self.cursor += 1;
		}
	}
}