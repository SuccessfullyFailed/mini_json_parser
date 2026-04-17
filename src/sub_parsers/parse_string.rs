use crate::{ Json, JsonParseResult, JsonSource, JsonTags };



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
			("'", "'", &[("\\", 1)]),
			("\"", "\"", &[("\\", 1)])
		])
	}
}



impl JsonSource for String {
	
	/// Turn the source into a json object.
	fn into_json(self) -> Json {
		Json::String(self)
	}
}
impl JsonSource for &str {
	
	/// Turn the source into a json object.
	fn into_json(self) -> Json {
		Json::String(self.to_string())
	}
}