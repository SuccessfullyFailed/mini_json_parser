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
	pub fn new<JsonSource>(json_obj:JsonSource) -> Json where Json:From<JsonSource> {
		Json::from(json_obj)
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



	/* CHILD METHODS */

	/// Try to get a child by index.
	/// Will only work on arrays or dictionaries with an integer as key.
	pub fn child_by_index(&self, index:usize) -> Option<&Json> {
		match self {
			Json::Array(items) => items.get(index),
			Json::Dict(_) => self.child_by_key(&Json::Int(index as i64)),
			_ => None
		}
	}

	/// Try to get mutable a child by index.
	/// Will only work on arrays or dictionaries with an integer as key.
	pub fn child_by_index_mut(&mut self, index:usize) -> Option<&mut Json> {
		match self {
			Json::Array(items) => items.get_mut(index),
			Json::Dict(_) => self.child_by_key_mut(&Json::Int(index as i64)),
			_ => None
		}
	}

	/// Try to get a child by key.
	/// Will only work on dictionaries.
	pub fn child_by_key(&self, key:&Json) -> Option<&Json> {
		match self {
			Json::Dict(items) => {
				if let Some((_, value)) = items.iter().find(|(item_key, _)| item_key == key) {
					if let Some(value) = value {
						return Some(value);
					}
				}
				None
			},
			_ => None
		}
	}

	/// Try to get a mutable child by key.
	/// Will only work on dictionaries.
	pub fn child_by_key_mut(&mut self, key:&Json) -> Option<&mut Json> {
		match self {
			Json::Dict(items) => {
				if let Some((_, value)) = items.iter_mut().find(|(item_key, _)| item_key == key) {
					if let Some(value) = value {
						return Some(value);
					}
				}
				None
			},
			_ => None
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
					"{}{}{}",
					tag_set.array_tags.open,
					items.iter().map(|item| item.to_json_string(tag_set)).collect::<Vec<String>>().join(tag_set.array_tags.separator),
					tag_set.array_tags.close
				)
			},
			Json::Dict(items) => {
				format!(
					"{}{}{}",
					tag_set.dict_tags.open,
					items.iter().map(|(key, value)|
						format!("{}{}", key.to_string(), value.as_ref().map(|value| tag_set.dict_tags.key_value_separator.to_string() + &value.to_json_string(tag_set)).unwrap_or_default())
					).collect::<Vec<String>>().join(tag_set.dict_tags.item_separator),
					tag_set.dict_tags.close
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