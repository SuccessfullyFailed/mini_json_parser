use std::{ error::Error, fmt::Debug };
use crate::sub_parsers::*;
use file_ref::FileRef;



// TODO: Split code here into separate files.


pub struct Json {
	json_object:Box<dyn JsonObj>,
	tags:JsonTagsSet
}
impl Json {

	/* CONSTRUCTOR METHODS */

	/// Create a new JSON struct from a json object.
	pub fn new<Source:JsonSource>(json_obj:Source) -> Json {
		Json::new_with_tag(json_obj, JsonTagsSet::default())
	}

	/// Create a new JSON struct from a json object with the specified tag set.
	pub fn new_with_tag<Source:JsonSource>(json_obj:Source, tags:JsonTagsSet) -> Json {
		Json {
			json_object: json_obj.into_json_obj(),
			tags
		}
	}

	/// Create a new JSON struct from a file.
	pub fn from_file(file_path:&str) -> Result<Json, Box<dyn Error>> {
		Json::from_file_with_tag_set(file_path, JsonTagsSet::default())
	}

	/// Create a new JSON struct from a file with the specified tag set.
	pub fn from_file_with_tag_set(file_path:&str, tag_set:JsonTagsSet) -> Result<Json, Box<dyn Error>> {
		let file_contents:String = FileRef::new(file_path).read()?;
		match Json::from_str_with_tag_set(&file_contents, tag_set) {
			Some(json) => Ok(json),
			None => Err("Could not parse file contents into json.".into())
		}
	}

	/// Create a new JSON struct from JSON contents.
	pub fn from_str(contents:&str) -> Option<Json> {
		Json::from_str_with_tag_set(contents, JsonTagsSet::default())
	}

	/// Create a new JSON struct from JSON contents with the specified tag set.
	pub fn from_str_with_tag_set(contents:&str, tags:JsonTagsSet) -> Option<Json> {
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
		self.json_object.to_json_str(&self.tags) == self.json_object.to_json_str(&other.tags)
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
	fn to_json_str(&self, tags:&JsonTagsSet) -> String;
}
impl JsonObj for Json {

	/// Get the name of the json-object type.
	fn json_type_name(&self) -> &str {
		self.json_object.json_type_name()
	}

	/// Convert the struct to a json string.
	fn to_json_str(&self, tags:&JsonTagsSet) -> String {
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



pub(crate) struct JsonParseResult {
	pub json:Box<dyn JsonObj>,
	pub match_length:usize
}
impl JsonParseResult {

	/// Create a new parse result.
	pub fn new<J:JsonObj>(json:J, match_length:usize) -> JsonParseResult {
		JsonParseResult {
			json: Box::new(json),
			match_length
		}
	}

	/// Try to get any result from the given str.
	pub fn try_any(contents:&str, tags:&JsonTagsSet) -> Option<JsonParseResult> {
		const PARSERS:&[fn(&str, &JsonTagsSet) -> Option<JsonParseResult>] = &[JsonBool::from_str, JsonNumber::from_str, JsonString::from_str, JsonArray::from_str, JsonDict::from_str];

		let whitespace_skip:usize = Self::whitespace_len(contents);
		let contents:&str = &contents[whitespace_skip..];
		for parser in PARSERS {
			if let Some(result) = parser(contents, tags) {
				return Some(JsonParseResult {
					json: result.json,
					match_length: result.match_length + whitespace_skip
				});
			}
		}
		None
	}

	/// Get the size of the leading whitespace.
	/// Returns 0 if no whitespace is found.
	pub fn whitespace_len(contents:&str) -> usize {
		contents.chars().take_while(|char| char.is_whitespace()).count()
	}
}



pub struct JsonTagsSet {
	pub bool_tags:JsonBoolTags,
	pub number_tags:JsonNumberTags,
	pub string_tags:JsonStringTags,
	pub array_tags:JsonArrayTags,
	pub dict_tags:JsonDictTags
}
impl JsonTagsSet {

	/// Create a new tags set.
	pub fn new(bool_tags:JsonBoolTags, number_tags:JsonNumberTags, string_tags:JsonStringTags, array_tags:JsonArrayTags, dict_tags:JsonDictTags) -> JsonTagsSet {
		JsonTagsSet {
			bool_tags,
			number_tags,
			string_tags,
			array_tags,
			dict_tags
		}
	}
}
impl Default for JsonTagsSet {
	fn default() -> Self {
		JsonTagsSet {
			bool_tags: JsonBoolTags::default(),
			number_tags: JsonNumberTags::default(),
			string_tags: JsonStringTags::default(),
			array_tags: JsonArrayTags::default(),
			dict_tags: JsonDictTags::default()
		}
	}
}