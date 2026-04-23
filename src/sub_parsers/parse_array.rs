use crate::{ Json, JsonParseResult, JsonTags };



impl JsonParseResult {

	/// Try to parse an Array JsonResult from a string.
	/// Assumes the provided str is trimmed.
	pub(crate) fn try_parse_array(content:&str, tags:&JsonTags) -> Option<JsonParseResult> {
		let array_tags:&JsonArrayTags = &tags.array_tags;
		if content.starts_with(array_tags.open) {
			let mut items:Vec<Json> = Vec::new();
			let mut cursor:usize = array_tags.open.len();
			let content_len:usize = content.len();
			while cursor < content_len {
				cursor += JsonParseResult::whitespace_len(&content[cursor..]);
				if cursor >= content_len {
					return None;
				}
				let remainder:&str = &content[cursor..];
				if remainder.starts_with(&array_tags.close) {
					cursor += array_tags.close.len();
					return Some(JsonParseResult::new(
						Json::Array(items),
						cursor
					))
				} else if remainder.starts_with(&array_tags.separator) {
					cursor += array_tags.separator.len();
					continue;
				} else if let Some(sub_match) = JsonParseResult::try_any(&content[cursor..], tags) {
					cursor += sub_match.match_length;
					items.push(sub_match.json);
				} else {
					return None;
				}
			}
			
		}
		None
	}
}



#[derive(Clone)]
pub struct JsonArrayTags {
	pub open:&'static str,
	pub separator:&'static str,
	pub close:&'static str
}
impl JsonArrayTags {

	/// Create a new JsonArrayTags set.
	pub const fn new(open:&'static str, separator:&'static str, close:&'static str) -> JsonArrayTags {
		JsonArrayTags {
			open,
			separator,
			close
		}
	}
}
impl Default for JsonArrayTags {
	fn default() -> Self {
		JsonArrayTags::new("[", ",", "]")
	}
}



impl<T> From<Vec<T>> for Json where Json:From<T> {
	fn from(value:Vec<T>) -> Self {
		Json::Array(
			value.into_iter().map(Json::from).collect()
		)
	}
}