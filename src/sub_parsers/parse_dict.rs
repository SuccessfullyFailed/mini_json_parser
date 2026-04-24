use crate::{ Json, JsonParseResult, JsonTags };
use std::error::Error;



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
			while let Some(key_json_result) = Self::try_any_dict_key(&content[cursor..], tags) {
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

	/// try to get any result from the given str.
	/// If no actual json is found, tries to parse a dict key without quote marks.
	fn try_any_dict_key(contents:&str, tags:&JsonTags) -> Option<JsonParseResult> {
		match JsonParseResult::try_any(contents, tags) {
			Some(result) => Some(result),
			None => {
				let contents_len:usize = contents.len();
				let key_val_separator:&str = tags.dict_tags.key_value_separator;
				let key_val_separator_len:usize = key_val_separator.len();

				let cursor_start:usize = Self::whitespace_len(contents);
				let mut cursor:usize = cursor_start;
				let cursor_end:usize = contents_len.max(key_val_separator_len) - key_val_separator_len;
				while cursor < cursor_end {
					if contents[cursor..].starts_with(&key_val_separator) {
						return Some(JsonParseResult::new(Json::DictKey(contents[cursor_start..cursor].trim().to_string()), cursor))
					}
					cursor += 1;
				}
				None
			}
		}
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
impl<'a, Key, Value> TryFrom<Json> for Vec<(Key, Option<Value>)> where Key:TryFrom<Json>, Value:TryFrom<Json> {
	type Error = Box<dyn Error>;

	fn try_from(value:Json) -> Result<Self, Self::Error> {
		match value {
			Json::Dict(items) => {
				let mut output:Vec<(Key, Option<Value>)> = Vec::new();
				for (item_index, (key, value)) in items.into_iter().enumerate() {
					match Key::try_from(key) {
						Ok(key) => match value {
							Some(value) => match Value::try_from(value) {
								Ok(value) => output.push((key, Some(value))),
								Err(_) => return Err(format!("The value of item {item_index} in the Json Dictionary could not be converted to the target value type.").into())
							},
							None => output.push((key, None))
						},
						Err(_) => return Err(format!("The key of item {item_index} in the Json Dictionary could not be converted to the target key type.").into())
					}
				}
				Ok(output)
			},
			_ => Err("Could not create a dictionary from a json value that is not a dictionary.".into())
		}
	}
}