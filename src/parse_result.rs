use crate::{ Json, JsonTags };



pub(crate) struct JsonParseResult {
	pub json:Json,
	pub match_length:usize
}
impl JsonParseResult {

	/// Create a new parse result.
	pub fn new<JsonSource>(json:JsonSource, match_length:usize) -> JsonParseResult where Json:From<JsonSource> {
		JsonParseResult {
			json: Json::from(json),
			match_length
		}
	}

	/// Try to get any result from the given str.
	pub fn try_any(contents:&str, tags:&JsonTags) -> Option<JsonParseResult> {
		const PARSERS:&[fn(&str, &JsonTags) -> Option<JsonParseResult>] = &[
			JsonParseResult::try_parse_bool,
			JsonParseResult::try_parse_number,
			JsonParseResult::try_parse_string,
			JsonParseResult::try_parse_array,
			JsonParseResult::try_parse_dict
		];

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