use crate::{ Json, JsonParseResult, JsonSource, JsonTags };



impl JsonParseResult {

	/// Try to parse a boolean JsonResult from a string.
	/// Assumes the provided str is trimmed.
	pub(crate) fn try_parse_bool(content:&str, tags:&JsonTags) -> Option<JsonParseResult> {
		JsonParseResult::bool_from_str(content, tags).map(|(value, match_length)| JsonParseResult::new(value, match_length))
	}

	/// try to parse a boolean from the given str.
	fn bool_from_str(content:&str, tags:&JsonTags) -> Option<(bool, usize)> {
		let bool_tags:&JsonBoolTags = &tags.bool_tags;

		// TODO: not great for performance.
		fn content_starts_with(content:&str, tag:&str) -> bool {
			content.len() >= tag.len() && content[..tag.len()].to_lowercase() == tag.to_lowercase()
		}

		if content_starts_with(content, bool_tags.true_tag) {
			return Some((true, bool_tags.true_tag.len()));
		}
		if content_starts_with(content, bool_tags.false_tag) {
			return Some((false, bool_tags.false_tag.len()));
		}
		if content_starts_with(content, bool_tags.flip_tag) {
			if let Some((sub_bool, sub_match_len)) = Self::bool_from_str(&content[bool_tags.flip_tag.len()..], tags) {
				return Some((!sub_bool, sub_match_len + bool_tags.flip_tag.len()));
			}
		}
		None
	}
}



#[derive(Clone)]
pub struct JsonBoolTags {
	pub true_tag:&'static str,
	pub false_tag:&'static str,
	pub flip_tag:&'static str
}
impl JsonBoolTags {

	/// Create a new JsonBoolTags set.
	pub const fn new(true_tag:&'static str, false_tag:&'static str, flip_tag:&'static str) -> JsonBoolTags {
		JsonBoolTags {
			true_tag,
			false_tag,
			flip_tag
		}
	}
}
impl Default for JsonBoolTags {
	fn default() -> Self {
		JsonBoolTags::new("true", "false", "!")
	}
}



impl JsonSource for bool {
	
	/// Turn the source into a json object.
	fn into_json(self) -> Json {
		Json::Bool(self)
	}
}