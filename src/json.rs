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
		Json::from_str_with_tag_set(&file_contents, tag_set)
	}

	/// Create a new JSON struct from JSON contents.
	pub fn from_str(contents:&str) -> Result<Json, Box<dyn Error>> {
		Json::from_str_with_tag_set(contents, &JsonTags::default())
	}

	/// Create a new JSON struct from JSON contents with the specified tag set.
	pub fn from_str_with_tag_set(contents:&str, tags:&JsonTags) -> Result<Json, Box<dyn Error>> {
		if let Some(json_result) = JsonParseResult::try_any(contents, tags) {
			Ok(json_result.json)
		} else {
			Err("Could not parse Json from given str.".into())
		}
	}



	/* CHILD METHODS */

	/// Try to get a child by type and selector.
	pub fn get<T, Selector>(&self, selector:Selector) -> Option<T> where T:TryFrom<Json>, Json:From<Selector> {
		if let Some(child) = self.get_json(Json::from(selector)) {
			if let Ok(child_value) = T::try_from(child.clone()) {
				return Some(child_value);
			}
		}
		None
	}

	/// Try to get a child json by selector.
	pub fn get_json(&self, selector:Json) -> Option<&Json> {
		match selector {

			// If the selector is an array, sub-select recursively using each selector.
			Json::Array(mut sub_selectors) => {
				if sub_selectors.is_empty() {
					None
				} else {
					let first_selector:Json = sub_selectors.remove(0);
					match self.get_json(first_selector) {
						Some(sub_selection) => {
							if sub_selectors.is_empty() {
								Some(sub_selection)
							} else {
								sub_selection.get_json(Json::Array(sub_selectors))
							}
						},
						None => None
					}
				}
			},

			// If the selector is not an array, sub-select based on the type of self.
			selector => {
				match self {

					// Try to get the child of an array by index.
					Json::Array(items) => {
						match selector {
							Json::Int(index) => items.get(index as usize),
							_ => None
						}
					},

					// Try to get a dictionary value by key.
					Json::Dict(items) => {
						match items.iter().find(|(item_key, _)| item_key == &selector) {
							Some((_key, value)) => match value {
								Some(value) => Some(value),
								None => None
							}
							None => None
						}
					},

					// Type of self does not allow child fetching.
					_ => None
				}
			}
		}
	}

	/// Try to get a mutable child json by selector.
	pub fn get_json_mut(&mut self, selector:Json) -> Option<&mut Json> {
		match selector {

			// If the selector is an array, sub-select recursively using each selector.
			Json::Array(mut sub_selectors) => {
				if sub_selectors.is_empty() {
					None
				} else {
					let first_selector:Json = sub_selectors.remove(0);
					match self.get_json_mut(first_selector) {
						Some(sub_selection) => {
							if sub_selectors.is_empty() {
								Some(sub_selection)
							} else {
								sub_selection.get_json_mut(Json::Array(sub_selectors))
							}
						},
						None => None
					}
				}
			},

			// If the selector is not an array, sub-select based on the type of self.
			selector => {
				match self {

					// Try to get the child of an array by index.
					Json::Array(items) => {
						match selector {
							Json::Int(index) => items.get_mut(index as usize),
							_ => None
						}
					},

					// Try to get a dictionary value by key.
					Json::Dict(items) => {
						match items.iter_mut().find(|(item_key, _)| item_key == &selector) {
							Some((_key, value)) => match value {
								Some(value) => Some(value),
								None => None
							}
							None => None
						}
					},

					// Type of self does not allow child fetching.
					_ => None
				}
			}
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