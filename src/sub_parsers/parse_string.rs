use crate::{ Json, JsonParseResult, JsonTags };
use std::error::Error;



impl JsonParseResult {

	/// Try to parse a string JsonResult from a string.
	/// Assumes the provided str is trimmed.
	pub(crate) fn try_parse_string(content:&str, tags:&JsonTags) -> Option<JsonParseResult> {
		let tags:&JsonStringTags = &tags.string_tags;
		for (open_tag, close_tag, escapes) in tags.quote_types {
			if content.starts_with(open_tag) {
				let mut cursor:usize = open_tag.len();
				let content_len:usize = content.len();
				let mut collected_string:String = String::new();
				while cursor < content_len {
					let remainder:&str = &content[cursor..];
					if remainder.starts_with(close_tag) {
						return Some(JsonParseResult::new(Json::String(collected_string), cursor + close_tag.len()));
					} else if let Some((escape_tag, escape_skip)) = escapes.iter().find(|(escape_tag, _)| remainder.starts_with(escape_tag)) {
						cursor += escape_tag.len();
						collected_string += &content[cursor..(cursor + escape_skip).min(content_len)];
						cursor += escape_skip;
					} else {
						collected_string += &content[cursor..cursor + 1];
						cursor += 1;
					}
				}
			}
		}
		None
	}
}



#[derive(Clone)]
pub struct JsonStringTags {
	pub quote_types:&'static [(&'static str, &'static str, &'static [(&'static str, usize)])] // Open tag, close tag, escapes tag and skip size
}
impl JsonStringTags {

	/// Create a new JsonStringTags set.
	pub const fn new(quote_types:&'static [(&'static str, &'static str, &[(&'static str, usize)])]) -> JsonStringTags {
		JsonStringTags {
			quote_types
		}
	}
}
impl Default for JsonStringTags {
	fn default() -> Self {
		JsonStringTags::new(&[
			("\"", "\"", &[("\\", 1)]),
			("'", "'", &[("\\", 1)])
		])
	}
}



impl From<String> for Json {
	fn from(value:String) -> Self {
		Json::String(value)
	}
}
impl From<&str> for Json {
	fn from(value:&str) -> Self {
		Json::String(value.to_string())
	}
}
impl TryFrom<Json> for String {
	type Error = Box<dyn Error>;
	
	fn try_from(value:Json) -> Result<Self, Self::Error> {
		match value {
			Json::String(value) => Ok(value),
			_ => Err("Could not create a string from a json value that is not a string.".into())
		}
	}
}
impl<'a> TryFrom<&'a Json> for &'a String {
	type Error = Box<dyn Error>;
	
	fn try_from(value:&'a Json) -> Result<Self, Self::Error> {
		match value {
			Json::String(value) => Ok(value),
			_ => Err("Could not create a string from a json value that is not a string.".into())
		}
	}
}
impl<'a> TryFrom<&'a mut Json> for &'a mut String {
	type Error = Box<dyn Error>;
	
	fn try_from(value:&'a mut Json) -> Result<Self, Self::Error> {
		match value {
			Json::String(value) => Ok(value),
			_ => Err("Could not create a string from a json value that is not a string.".into())
		}
	}
}
impl<'a> TryFrom<&'a Json> for &'a str {
	type Error = Box<dyn Error>;
	
	fn try_from(value:&'a Json) -> Result<Self, Self::Error> {
		match value {
			Json::String(value) => Ok(value.as_str()),
			_ => Err("Could not create a string from a json value that is not a string.".into())
		}
	}
}
impl<'a> TryFrom<&'a mut Json> for &'a mut str {
	type Error = Box<dyn Error>;
	
	fn try_from(value:&'a mut Json) -> Result<Self, Self::Error> {
		match value {
			Json::String(value) => Ok(value.as_mut_str()),
			_ => Err("Could not create a string from a json value that is not a string.".into())
		}
	}
}