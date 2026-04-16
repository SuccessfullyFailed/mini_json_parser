use crate::{ JsonObj, JsonParseResult, JsonSource, JsonTagsSet };



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



pub struct JsonString {
	content:String,
	open_quote:String,
	close_quote:String
}
impl JsonString {

	/// Create a new json string.
	pub fn new(content:&str, open_quote:&str, close_quote:&str) -> JsonString {
		JsonString {
			content: content.to_string(),
			open_quote: open_quote.to_string(),
			close_quote: close_quote.to_string()
		}
	}

	/// Try to parse a JsonResult from a string.
	/// Assumes the provided str is trimmed.
	pub(crate) fn from_str(content:&str, tags:&JsonTagsSet) -> Option<JsonParseResult> {
		let tags:&JsonStringTags = &tags.string_tags;
		for (open_tag, close_tag, escapes) in tags.quote_types {
			if content.starts_with(open_tag) {
				let mut cursor:usize = open_tag.len();
				let content_len:usize = content.len();
				while cursor < content_len {
					let remainder:&str = &content[cursor..];
					if remainder.starts_with(close_tag) {
						return Some(JsonParseResult::new(
							JsonString {
								content: content[open_tag.len()..cursor].to_string(),
								open_quote: content[..open_tag.len()].to_string(),
								close_quote: content[cursor..cursor + close_tag.len()].to_string()
							},
							cursor + close_tag.len()
						));
					}
					for (escape_tag, escape_skip) in *escapes {
						if remainder.starts_with(escape_tag) {
							cursor += escape_tag.len() + escape_skip - 1; // Remove one for the cursor incrementation at the end of the loop.
						}
					}
					cursor += 1;
				}
			}
		}
		None
	}
}
impl JsonObj for JsonString {

	/// Get the name of the json-object type.
	fn json_type_name(&self) -> &str {
		"string"
	}

	/// Convert the struct to a json string.
	fn to_json_str(&self, _tags:&JsonTagsSet) -> String {
		// TODO: implement optional quotes. take quotes from tags and escape content.
		format!("{}{}{}", self.open_quote, self.content, self.close_quote)
	}
}
impl JsonSource for String {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(JsonString::new(&self, "\"", "\""))
	}
}
impl JsonSource for &str {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(JsonString::new(self, "\"", "\""))
	}
}