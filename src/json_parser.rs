use crate::{ OpenCloseEscape, OpenCloseEscapeList };



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
			let type_parser_methods:&[fn(&mut JsonParser<'a>) -> Option<Json>] = &[Self::parse_bool, Self::parse_number, Self::parse_string];
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
		const SPACING_CHARS:&[char] = &['_', ','];
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
		const STRING_MATCHERS:OpenCloseEscapeList = OpenCloseEscapeList::new(&[
			OpenCloseEscape::new("'", "'", &[("\\", 1)]),
			OpenCloseEscape::new("\"", "\"", &[("\\", 1)])
		]);
		if self.cursor < self.contents_len {
			if let Some((match_body, match_length)) = STRING_MATCHERS.match_str(&self.contents[self.cursor..]) {
				self.cursor += match_length;
				return Some(Json::String(match_body.to_string()));
			}
		}
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