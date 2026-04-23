use crate::{ Json, JsonParseResult, JsonTags };



impl JsonParseResult {

	/// Try to parse a dictionary JsonResult from a string.
	/// Assumes the provided str is trimmed.
	pub(crate) fn try_parse_dict(content:&str, tags:&JsonTags) -> Option<JsonParseResult> {
		let dict_tags:&JsonDictTags = &tags.dict_tags;
		
		if content.starts_with(dict_tags.open) {
			let mut cursor:usize = dict_tags.open.len();
			cursor += JsonParseResult::whitespace_len(&content[cursor..]);

			// Keep collection json keys.
			let mut items:Vec<(Json, Option<Json>)> = Vec::new();
			let content_len:usize = content.len();
			while let Some(key_json_result) = JsonParseResult::try_any(&content[cursor..], tags) {
				let key:Json = key_json_result.json;
				cursor += key_json_result.match_length;
				cursor += JsonParseResult::whitespace_len(&content[cursor..]);

				// Match value to the key.
				let value_json:Option<Json> = {
					if cursor < content_len && content[cursor..].starts_with(dict_tags.key_value_separator) {
						cursor += dict_tags.key_value_separator.len();
						if cursor < content_len {
							match JsonParseResult::try_any(&content[cursor..], tags) {
								Some(value_json_result) => {
									cursor += value_json_result.match_length;
									Some(value_json_result.json)
								},
								None => None
							}
						} else {
							None
						}
					} else {
						None
					}
				};
				items.push((key, value_json));
				cursor += JsonParseResult::whitespace_len(&content[cursor..]);

				// Continue matching more values, end the dict or conclude invalid dict.
				if cursor >= content_len {
					return None;
				} else if content[cursor..].starts_with(&dict_tags.item_separator) {
					cursor += dict_tags.item_separator.len();
				} else if content[cursor..].starts_with(&dict_tags.close) {
					break
				} else {
					return None;
				}
			}
			if cursor < content_len && content[cursor..].starts_with(dict_tags.close) {
				cursor += dict_tags.close.len();
				return Some(JsonParseResult::new(Json::Dict(items), cursor));
			}
		}
		None
	}
}



#[derive(Clone)]
pub struct JsonDictTags {
	pub open:&'static str,
	pub key_value_separator:&'static str,
	pub item_separator:&'static str,
	pub close:&'static str
}
impl JsonDictTags {

	/// Create a new JsonDictTags set.
	pub const fn new(open:&'static str, key_value_separator:&'static str, item_separator:&'static str, close:&'static str) -> JsonDictTags {
		JsonDictTags {
			open,
			key_value_separator,
			item_separator,
			close
		}
	}
}
impl Default for JsonDictTags {
	fn default() -> Self {
		JsonDictTags::new("{", ":", ",", "}")
	}
}



impl<Key, Value> From<Vec<(Key, Option<Value>)>> for Json where Json:From<Key> + From<Value> {
	fn from(value:Vec<(Key, Option<Value>)>) -> Self {
		Json::Dict(
			value.into_iter().map(|(key, value)| (Json::from(key), value.map(Json::from))).collect()
		)
	}
}
impl<Key, Value> From<Vec<(Key, Value)>> for Json where Json:From<Key> + From<Value> {
	fn from(value:Vec<(Key, Value)>) -> Self {
		Json::Dict(
			value.into_iter().map(|(key, value)| (Json::from(key), Some(Json::from(value)))).collect()
		)
	}
}