use crate::{ JsonObj, JsonParseResult, JsonSource, JsonTags };



#[derive(Clone)]
pub struct JsonNumberTags {
	pub decimal_separator:&'static str,
	pub negative:&'static str,
	pub decorations:&'static [&'static str]
}
impl JsonNumberTags {

	/// Create a new JsonNumberTags set.
	pub const fn new(decimal_separator:&'static str, negative:&'static str, decoration:&'static [&'static str]) -> JsonNumberTags {
		JsonNumberTags {
			decimal_separator,
			negative,
			decorations: decoration
		}
	}
}
impl Default for JsonNumberTags {
	fn default() -> Self {
		JsonNumberTags::new(".", "-", &["_"])
	}
}



pub struct JsonFloat(pub f64);
pub struct JsonInt(pub i64);
pub struct JsonNumber;
impl JsonNumber {

	/// Create a new json number.
	pub fn new<Number:JsonSource>(number:Number) -> Box<dyn JsonObj> {
		number.into_json_obj()
	}

	/// Try to parse a JsonResult from a string.
	/// Assumes the provided str is trimmed.
	pub(crate) fn from_str(content:&str, tags:&JsonTags) -> Option<JsonParseResult> {
		let tags:&JsonNumberTags = &tags.number_tags;

		let content_len:usize = content.len();
		let mut cursor:usize = 0;

		// Parse negative flip.
		let mut negative_flip:bool = false;
		while cursor < content_len && content[cursor..].starts_with(tags.negative) {
			negative_flip = !negative_flip;
			cursor += tags.negative.len();
		}

		// Parse number.
		let number_start:usize = cursor;
		let mut remainder:&str = &content[cursor..];
		while !remainder.is_empty() {
			if remainder.chars().next().is_some_and(|char| char.is_numeric()) {
				cursor += 1;
			} else if remainder.starts_with(tags.decimal_separator) {
				cursor +=  tags.decimal_separator.len();
			} else if let Some(decoration) = tags.decorations.iter().find(|decoration| remainder.starts_with(*decoration)) {
				cursor += decoration.len();
			} else {
				break;
			}
			remainder = &content[cursor..];
		}

		// Figure out number type.
		let mut number_str:String = content[number_start..cursor].to_string();
		for decoration in tags.decorations {
			number_str = number_str.replace(decoration, "");
		}
		if !number_str.chars().any(|char| char.is_numeric()) {
			None
		} else if number_str.contains(tags.decimal_separator) {
			Some(JsonParseResult::new(JsonFloat(number_str.replace(tags.decimal_separator, ".").parse::<f64>().unwrap()), cursor))
		} else {
			Some(JsonParseResult::new(JsonInt(number_str.parse().unwrap()), cursor))
		}
	}
}
impl JsonObj for JsonFloat {

	/// Get the name of the json-object type.
	fn json_type_name(&self) -> &str {
		"float"
	}

	/// Convert the struct to a json string.
	fn to_json_str(&self, tags:&JsonTags) -> String {
		self.0.to_string().replace(".", tags.number_tags.decimal_separator)
	}
}
impl JsonObj for JsonInt {

	/// Get the name of the json-object type.
	fn json_type_name(&self) -> &str {
		"integer"
	}

	/// Convert the struct to a json string.
	fn to_json_str(&self, _tags:&JsonTags) -> String {
		self.0.to_string()
	}
}



impl JsonSource for f64 {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(JsonFloat(self))
	}
}
impl JsonSource for f32 {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(JsonFloat(self as f64))
	}
}
impl JsonSource for i64 {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(JsonInt(self))
	}
}
impl JsonSource for i32 {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(JsonInt(self as i64))
	}
}
impl JsonSource for i16 {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(JsonInt(self as i64))
	}
}
impl JsonSource for i8 {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(JsonInt(self as i64))
	}
}
impl JsonSource for u64 {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(JsonInt(self as i64))
	}
}
impl JsonSource for u32 {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(JsonInt(self as i64))
	}
}
impl JsonSource for u16 {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(JsonInt(self as i64))
	}
}
impl JsonSource for u8 {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(JsonInt(self as i64))
	}
}