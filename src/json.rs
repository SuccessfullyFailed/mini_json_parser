use crate::{ JsonParseResult, JsonTags };
use std::{ error::Error, fmt::Debug };
use file_ref::FileRef;



// TODO: Split code here into separate files.


pub struct Json {
	json_object:Box<dyn JsonObj>,
	tags:JsonTags
}
impl Json {

	/* CONSTRUCTOR METHODS */

	/// Create a new JSON struct from a json object.
	pub fn new<Source:JsonSource>(json_obj:Source) -> Json {
		Json::new_with_tag(json_obj, JsonTags::default())
	}

	/// Create a new JSON struct from a json object with the specified tag set.
	pub fn new_with_tag<Source:JsonSource>(json_obj:Source, tags:JsonTags) -> Json {
		Json {
			json_object: json_obj.into_json_obj(),
			tags
		}
	}

	/// Create a new JSON struct from a file.
	pub fn from_file(file_path:&str) -> Result<Json, Box<dyn Error>> {
		Json::from_file_with_tag_set(file_path, JsonTags::default())
	}

	/// Create a new JSON struct from a file with the specified tag set.
	pub fn from_file_with_tag_set(file_path:&str, tag_set:JsonTags) -> Result<Json, Box<dyn Error>> {
		let file_contents:String = FileRef::new(file_path).read()?;
		match Json::from_str_with_tag_set(&file_contents, tag_set) {
			Some(json) => Ok(json),
			None => Err("Could not parse file contents into json.".into())
		}
	}

	/// Create a new JSON struct from JSON contents.
	pub fn from_str(contents:&str) -> Option<Json> {
		Json::from_str_with_tag_set(contents, JsonTags::default())
	}

	/// Create a new JSON struct from JSON contents with the specified tag set.
	pub fn from_str_with_tag_set(contents:&str, tags:JsonTags) -> Option<Json> {
		if let Some(json_result) = JsonParseResult::try_any(contents, &tags) {
			Some(Json {
				json_object: json_result.json,
				tags
			})
		} else {
			None
		}
	}
}
impl ToString for Json {
	fn to_string(&self) -> String {
		self.json_object.to_json_str(&self.tags)
	}
}
impl PartialEq for Json {
	fn eq(&self, other:&Self) -> bool {
		self.json_object.json_type_name() == other.json_object.json_type_name() &&
		self.json_object.to_json_str(&self.tags) == self.json_object.to_json_str(&self.tags) // Two json objects built from different tags are still the same object
	}
}
impl Debug for Json {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.to_string())
	}
}



pub trait JsonObj:Send + Sync + 'static {

	/// Get the name of the json-object type.
	fn json_type_name(&self) -> &str;

	/// Convert the struct to a json string.
	fn to_json_str(&self, tags:&JsonTags) -> String;
}
impl JsonObj for Json {

	/// Get the name of the json-object type.
	fn json_type_name(&self) -> &str {
		self.json_object.json_type_name()
	}

	/// Convert the struct to a json string.
	fn to_json_str(&self, tags:&JsonTags) -> String {
		self.json_object.to_json_str(tags)
	}
}



pub trait JsonSource {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj>;
}
impl<T:JsonObj + 'static> JsonSource for T {
	
	/// Turn the source into a json object.
	fn into_json_obj(self) -> Box<dyn JsonObj> {
		Box::new(self)
	}
}