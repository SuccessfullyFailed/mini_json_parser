use crate::{ JsonObj, JsonParseResult, JsonSource, JsonTags };



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



pub struct JsonDict(Vec<(Box<dyn JsonObj>, Option<Box<dyn JsonObj>>)>);
impl JsonDict {

	/// Create a new json array.
	pub fn new(items:Vec<(Box<dyn JsonObj>, Option<Box<dyn JsonObj>>)>) -> JsonDict {
		JsonDict(items)
	}

	/// Return the array with an additional item.
	pub fn with_item<KeyJson:JsonObj, ValueJson:JsonObj>(mut self, key:KeyJson, value:Option<ValueJson>) -> Self {
		self.add_item(key, value);
		self
	}

	/// Add an item to the array.
	pub fn add_item<KeyJson:JsonObj, Value:JsonObj>(&mut self, key:KeyJson, value:Option<Value>) {
		self.0.push((
			Box::new(key),
			match value {
				Some(value) => Some(Box::new(value)),
				None => None
			}
		));
	}

	/// Try to parse a JsonResult from a string.
	/// Assumes the provided str is trimmed.
	pub(crate) fn from_str(content:&str, tags:&JsonTags) -> Option<JsonParseResult> {
		let dict_tags:&JsonDictTags = &tags.dict_tags;
		
		if content.starts_with(dict_tags.open) {
			let mut cursor:usize = dict_tags.open.len();
			cursor += JsonParseResult::whitespace_len(&content[cursor..]);

			// Keep collection json keys.
			let mut items:Vec<(Box<dyn JsonObj>, Option<Box<dyn JsonObj>>)> = Vec::new();
			let content_len:usize = content.len();
			while let Some(key_json_result) = JsonParseResult::try_any(&content[cursor..], tags) {
				let key:Box<dyn JsonObj> = key_json_result.json;
				cursor += key_json_result.match_length;
				cursor += JsonParseResult::whitespace_len(&content[cursor..]);

				// Match value to the key.
				let value_json:Option<Box<dyn JsonObj>> = {
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
				return Some(JsonParseResult::new(JsonDict(items), cursor));
			}
		}
		None
	}
}
impl JsonObj for JsonDict {

	/// Get the name of the json-object type.
	fn json_type_name(&self) -> &str {
		"dictionary"
	}

	/// Convert the struct to a json string.
	fn to_json_str(&self, tags:&JsonTags) -> String {
		format!(
			"{}{}{}",
			tags.dict_tags.open,
			self.0.iter().map(|(key, value)|
				format!("{}{}", key.to_json_str(tags), value.as_ref().map(|value| ":".to_string() + &value.to_json_str(tags)).unwrap_or_default())
			).collect::<Vec<String>>().join(tags.dict_tags.item_separator),
			tags.dict_tags.close
		)
	}
}
impl JsonSource for Vec<(Box<dyn JsonObj>, Option<Box<dyn JsonObj>>)> {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(JsonDict::new(self))
	}
}
impl JsonSource for Vec<(Box<dyn JsonObj>, Box<dyn JsonObj>)> {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(JsonDict::new(
			self.into_iter().map(|(key, value)| (key, Some(value))).collect()
		))
	}
}
impl<Key:JsonSource, Value:JsonSource> JsonSource for Vec<(Key, Option<Value>)> {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(JsonDict::new(
			self.into_iter().map(|(key, value)| (key.into_json_obj(), value.map(|value| value.into_json_obj()))).collect()
		))
	}
}
impl<Key:JsonSource, Value:JsonSource> JsonSource for Vec<(Key, Value)> {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(JsonDict::new(
			self.into_iter().map(|(key, value)| (key.into_json_obj(), Some(value.into_json_obj()))).collect()
		))
	}
}