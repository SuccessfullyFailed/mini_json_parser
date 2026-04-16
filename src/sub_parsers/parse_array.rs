use crate::{ JsonObj, JsonParseResult, JsonSource, JsonTagsSet };



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



pub struct JsonArray(Vec<Box<dyn JsonObj>>);
impl JsonArray {

	/// Create a new json array.
	pub fn new(items:Vec<Box<dyn JsonObj>>) -> JsonArray {
		JsonArray(items)
	}

	/// Return the array with an additional item.
	pub fn with_item<ItemJson:JsonObj>(mut self, item:ItemJson) -> Self {
		self.add_item(item);
		self
	}

	/// Add an item to the array.
	pub fn add_item<ItemJson:JsonObj>(&mut self, item:ItemJson) {
		self.0.push(Box::new(item));
	}

	/// Try to parse a JsonResult from a string.
	/// Assumes the provided str is trimmed.
	pub(crate) fn from_str(content:&str, tags:&JsonTagsSet) -> Option<JsonParseResult> {
		let array_tags:&JsonArrayTags = &tags.array_tags;
		if content.starts_with(array_tags.open) {
			let mut items:Vec<Box<dyn JsonObj>> = Vec::new();
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
					break;
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
			Some(JsonParseResult::new(
				JsonArray(items),
				cursor
			))
		} else {
			None
		}
	}
}
impl JsonObj for JsonArray {

	/// Get the name of the json-object type.
	fn json_type_name(&self) -> &str {
		"array"
	}

	/// Convert the struct to a json string.
	fn to_json_str(&self, tags:&JsonTagsSet) -> String {
		format!(
			"{}{}{}",
			tags.array_tags.open,
			self.0.iter().map(|item| item.to_json_str(tags)).collect::<Vec<String>>().join(tags.array_tags.separator),
			tags.array_tags.close
		)
	}
}
impl JsonSource for Vec<Box<dyn JsonObj>> {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(JsonArray::new(self))
	}
}
impl<T:JsonSource> JsonSource for Vec<T> {

	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(JsonArray::new(
			self.into_iter().map(|value| value.into_json_obj()).collect()
		))
	}
}