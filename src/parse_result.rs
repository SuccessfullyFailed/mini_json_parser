use crate::{ JsonObj, JsonTags, sub_parsers::* };



pub(crate) struct JsonParseResult {
	pub json:Box<dyn JsonObj>,
	pub match_length:usize
}
impl JsonParseResult {

	/// Create a new parse result.
	pub fn new<J:JsonObj>(json:J, match_length:usize) -> JsonParseResult {
		JsonParseResult {
			json: Box::new(json),
			match_length
		}
	}

	/// Try to get any result from the given str.
	pub fn try_any(contents:&str, tags:&JsonTags) -> Option<JsonParseResult> {
		const PARSERS:&[fn(&str, &JsonTags) -> Option<JsonParseResult>] = &[JsonBool::from_str, JsonNumber::from_str, JsonString::from_str, JsonArray::from_str, JsonDict::from_str];

		let whitespace_skip:usize = Self::whitespace_len(contents);
		let contents:&str = &contents[whitespace_skip..];
		for parser in PARSERS {
			if let Some(result) = parser(contents, tags) {
				return Some(JsonParseResult {
					json: result.json,
					match_length: result.match_length + whitespace_skip
				});
			}
		}
		None
	}

	/// Get the size of the leading whitespace.
	/// Returns 0 if no whitespace is found.
	pub fn whitespace_len(contents:&str) -> usize {
		contents.chars().take_while(|char| char.is_whitespace()).count()
	}
}