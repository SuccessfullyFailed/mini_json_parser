use crate::{ JsonParseResult, JsonTags };
use std::{ error::Error, fmt::Debug };
use file_ref::FileRef;



#[derive(PartialEq, Debug, Clone)]
pub enum Json {
	Bool(bool),
	Int(i64),
	Float(f64),
	String(String),
	Array(Vec<Json>),
	Dict(Vec<(Json, Option<Json>)>)
}
impl Json {

	/* CONSTRUCTOR METHODS */

	/// Create a new JSON struct from a json object.
	pub fn new<Source:JsonSource>(json_obj:Source) -> Json {
		json_obj.into_json()
	}

	/// Create a new JSON struct from a file.
	pub fn from_file(file_path:&str) -> Result<Json, Box<dyn Error>> {
		Json::from_file_with_tag_set(file_path, &JsonTags::default())
	}

	/// Create a new JSON struct from a file with the specified tag set.
	pub fn from_file_with_tag_set(file_path:&str, tag_set:&JsonTags) -> Result<Json, Box<dyn Error>> {
		let file_contents:String = FileRef::new(file_path).read()?;
		match Json::from_str_with_tag_set(&file_contents, tag_set) {
			Some(json) => Ok(json),
			None => Err("Could not parse file contents into json.".into())
		}
	}

	/// Create a new JSON struct from JSON contents.
	pub fn from_str(contents:&str) -> Option<Json> {
		Json::from_str_with_tag_set(contents, &JsonTags::default())
	}

	/// Create a new JSON struct from JSON contents with the specified tag set.
	pub fn from_str_with_tag_set(contents:&str, tags:&JsonTags) -> Option<Json> {
		if let Some(json_result) = JsonParseResult::try_any(contents, tags) {
			Some(json_result.json)
		} else {
			None
		}
	}



	/* USAGE METHODS */

	/// Convert the json to a string.
	/// Requires a tags set.
	/// Normal to-string method is available, which will assume the default tags set.
	pub fn to_json_string(&self, tag_set:&JsonTags) -> String {
		match self {
			Json::Bool(value) => {
				if *value {
					tag_set.bool_tags.true_tag.to_string()
				} else {
					tag_set.bool_tags.false_tag.to_string()
				}
			},

			Json::Int(value) => {
				format!(
					"{}{}",
					if *value < 0 { &tag_set.number_tags.negative } else { "" },
					value.abs()
				)
			},

			Json::Float(value) => {
				format!(
					"{}{}{}",
					Json::Int(value.round() as i64).to_string(),
					&tag_set.number_tags.decimal_separator,
					&value.fract().to_string()[2..]
				)
			},

			Json::String(string) => {
				format!(
					"{}{}{}",
					tag_set.string_tags.quote_types[0].0,
					string, // TODO: Add automated escaping
					tag_set.string_tags.quote_types[0].1
				)
			},

			Json::Array(items) => {
				format!(
					"[{}]",
					items.iter().map(|item| item.to_json_string(tag_set)).collect::<Vec<String>>().join(", ")
				)
			},
			Json::Dict(items) => {
				format!(
					"[{}]",
					items.iter().map(|(key, value)|
						format!("{}{}", key.to_string(), value.as_ref().map(|value| ":".to_string() + &value.to_json_string(tag_set)).unwrap_or_default())
					).collect::<Vec<String>>().join(", ")
				)
			}
		}
	}
}
impl ToString for Json {
	fn to_string(&self) -> String {
		self.to_json_string(&JsonTags::default())
	}
}



pub trait JsonSource {
	
	/// Turn the source into a json object.
	fn into_json(self) -> Json;
}
impl JsonSource for Json {

	/// Turn the source into a json object.
	fn into_json(self) -> Json {
		self
	}
}