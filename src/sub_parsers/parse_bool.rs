use crate::{ JsonObj, JsonParseResult, JsonSource, JsonTagsSet };



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



pub struct JsonBool(bool);
impl JsonBool {

	/// Create a new JsonBool.
	pub const fn new(boolean:bool) -> JsonBool {
		JsonBool(boolean)
	}

	/// Try to parse a JsonResult from a string.
	/// Assumes the provided str is trimmed.
	pub(crate) fn from_str(content:&str, tags:&JsonTagsSet) -> Option<JsonParseResult> {
		Self::bool_from_str(content, tags).map(|(boolean, match_length)| JsonParseResult::new(JsonBool(boolean), match_length))
	}

	/// try to parse a boolean from the given str.
	fn bool_from_str(content:&str, tags:&JsonTagsSet) -> Option<(bool, usize)> {

		// TODO: not great for performance.
		fn content_starts_with(content:&str, tag:&str) -> bool {
			content.len() >= tag.len() && content[..tag.len()].to_lowercase() == tag.to_lowercase()
		}

		if content_starts_with(content, tags.bool_tags.true_tag) {
			return Some((true, tags.bool_tags.true_tag.len()));
		}
		if content_starts_with(content, tags.bool_tags.false_tag) {
			return Some((false, tags.bool_tags.false_tag.len()));
		}
		if content_starts_with(content, tags.bool_tags.flip_tag) && content.len() > tags.bool_tags.flip_tag.len() {
			if let Some((sub_bool, sub_match_len)) = Self::bool_from_str(&content[1..], tags) {
				return Some((!sub_bool, sub_match_len + tags.bool_tags.flip_tag.len()));
			}
		}
		None
	}
}
impl JsonObj for JsonBool {

	/// Get the name of the json-object type.
	fn json_type_name(&self) -> &str {
		"boolean"
	}

	/// Convert the struct to a json string.
	fn to_json_str(&self, tags:&JsonTagsSet) -> String {
		if self.0 {
			tags.bool_tags.true_tag.to_string()
		} else {
			tags.bool_tags.false_tag.to_string()
		}
	}
}
impl JsonSource for bool {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(JsonBool(self))
	}
}