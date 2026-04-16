use crate::sub_parsers::*;



#[derive(Clone)]
pub struct JsonTags {
	pub bool_tags:JsonBoolTags,
	pub number_tags:JsonNumberTags,
	pub string_tags:JsonStringTags,
	pub array_tags:JsonArrayTags,
	pub dict_tags:JsonDictTags
}
impl JsonTags {

	/// Create a new tags set.
	pub fn new(bool_tags:JsonBoolTags, number_tags:JsonNumberTags, string_tags:JsonStringTags, array_tags:JsonArrayTags, dict_tags:JsonDictTags) -> JsonTags {
		JsonTags {
			bool_tags,
			number_tags,
			string_tags,
			array_tags,
			dict_tags
		}
	}
}
impl Default for JsonTags {
	fn default() -> Self {
		JsonTags {
			bool_tags: JsonBoolTags::default(),
			number_tags: JsonNumberTags::default(),
			string_tags: JsonStringTags::default(),
			array_tags: JsonArrayTags::default(),
			dict_tags: JsonDictTags::default()
		}
	}
}