use crate::{ Json, JsonParseResult, JsonTags };
use std::error::Error;



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
impl<T> TryFrom<Json> for Vec<T> where T:TryFrom<Json> {
	type Error = Box<dyn Error>;

	fn try_from(value:Json) -> Result<Self, Self::Error> {
		match value {
			Json::Array(items) => {
				let mut output:Vec<T> = Vec::new();
				for (item_index, item) in items.into_iter().enumerate() {
					match T::try_from(item) {
						Ok(value) => output.push(value),
						Err(_) => return Err(format!("Item {item_index} in the Json Array could not be converted to the target type.").into())
					}
				}
				Ok(output)
			},
			_ => Err("Could not create an array from a json value that is not an array.".into())
		}
	}
}
impl<'a, T> TryFrom<&'a Json> for Vec<&'a T> where &'a T:TryFrom<&'a Json> {
	type Error = Box<dyn Error>;
	
	fn try_from(value:&'a Json) -> Result<Self, Self::Error> {
		match value {
			Json::Array(value) => {
				let mut output:Vec<&'a T> = Vec::new();
				for (item_index, item) in value.into_iter().enumerate() {
					match <&'a T>::try_from(item) {
						Ok(value) => output.push(value),
						Err(_) => return Err(format!("Item {item_index} in the Json Array could not be converted to the target type.").into())
					}
				}
				Ok(output)
			},
			_ => Err("Could not create a string from a json value that is not a string.".into())
		}
	}
}
impl<'a, T> TryFrom<&'a mut Json> for Vec<&'a mut T> where &'a mut T:TryFrom<&'a mut Json> {
	type Error = Box<dyn Error>;
	
	fn try_from(value:&'a mut Json) -> Result<Self, Self::Error> {
		match value {
			Json::Array(value) => {
				let mut output:Vec<&'a mut T> = Vec::new();
				for (item_index, item) in value.into_iter().enumerate() {
					match <&'a mut T>::try_from(item) {
						Ok(value) => output.push(value),
						Err(_) => return Err(format!("Item {item_index} in the Json Array could not be converted to the target type.").into())
					}
				}
				Ok(output)
			},
			_ => Err("Could not create a string from a json value that is not a string.".into())
		}
	}
}