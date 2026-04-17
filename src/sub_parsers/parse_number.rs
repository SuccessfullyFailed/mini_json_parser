use crate::{ Json, JsonParseResult, JsonSource, JsonTags };



impl JsonParseResult {

	/// Try to parse a number JsonResult from a string.
	/// Assumes the provided str is trimmed.
	pub(crate) fn try_parse_number(content:&str, tags:&JsonTags) -> Option<JsonParseResult> {
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
		} else if !number_str.contains(tags.decimal_separator) {
			let value:i64 = number_str.parse::<i64>().unwrap() * if negative_flip { -1 } else { 1 };
			Some(JsonParseResult::new(Json::Int(value), cursor))
		} else if let [rounded, fraction] = number_str.split(tags.decimal_separator).collect::<Vec<&str>>()[..] {
			let rounded:i64 = rounded.parse::<i64>().unwrap();
			let fraction:f64 = format!("0.{fraction}").parse::<f64>().unwrap();
			let value:f64 = (rounded as f64 + fraction) * if negative_flip { -1.0 } else { 1.0 };
			Some(JsonParseResult::new(Json::Float(value), cursor))
		} else {
			None
		}
	}
}



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


impl JsonSource for f64 {
	
	/// Turn the source into a json object.
	fn into_json(self) -> Json {
		Json::Float(self)
	}
}
impl JsonSource for f32 {
	
	/// Turn the source into a json object.
	fn into_json(self) -> Json {
		Json::Float(self as f64)
	}
}
impl JsonSource for i64 {
	
	/// Turn the source into a json object.
	fn into_json(self) -> Json {
		Json::Int(self)
	}
}
impl JsonSource for i32 {
	
	/// Turn the source into a json object.
	fn into_json(self) -> Json {
		Json::Int(self as i64)
	}
}
impl JsonSource for i16 {
	
	/// Turn the source into a json object.
	fn into_json(self) -> Json {
		Json::Int(self as i64)
	}
}
impl JsonSource for i8 {
	
	/// Turn the source into a json object.
	fn into_json(self) -> Json {
		Json::Int(self as i64)
	}
}
impl JsonSource for u64 {
	
	/// Turn the source into a json object.
	fn into_json(self) -> Json {
		Json::Int(self as i64)
	}
}
impl JsonSource for u32 {
	
	/// Turn the source into a json object.
	fn into_json(self) -> Json {
		Json::Int(self as i64)
	}
}
impl JsonSource for u16 {
	
	/// Turn the source into a json object.
	fn into_json(self) -> Json {
		Json::Int(self as i64)
	}
}
impl JsonSource for u8 {
	
	/// Turn the source into a json object.
	fn into_json(self) -> Json {
		Json::Int(self as i64)
	}
}