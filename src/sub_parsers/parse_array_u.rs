#[cfg(test)]
mod tests {
	use crate::{ Json, JsonArrayTags, JsonTags };



	/* DEFAULT TAG SET */
	
	#[test]
	fn can_parse_array() {
		assert_eq!(Json::from_str("[]"), Some(Json::new(Vec::<u8>::new())));
		assert_eq!(Json::from_str("[0,2,4,8]"), Some(Json::new(vec![0, 2, 4, 8])));
		assert_eq!(Json::from_str("[0,[1,2,3],true]"), Some(Json::new(vec![Json::new(0), Json::new(vec![1, 2, 3]), Json::new(true)])));
	}
	
	#[test]
	fn can_parse_array_with_whitespace() {
		assert_eq!(Json::from_str("\n\t [\n\t ]\n\t "), Some(Json::new(Vec::<u8>::new())));
		assert_eq!(Json::from_str("\n\t [0, 2, 4, 8]\n\t "), Some(Json::new(vec![0, 2, 4, 8])));
		assert_eq!(Json::from_str("\n\t [0, [1, 2, 3], true]\n\t "), Some(Json::new(vec![Json::new(0), Json::new(vec![1, 2, 3]), Json::new(true)])));
	}

	#[test]
	fn can_not_parse_invalid_array() {
		assert_eq!(Json::from_str("['array_without_end'"), None);
		assert_eq!(Json::from_str("]"), None);
		assert_eq!(Json::from_str("['broken_sub_array', []"), None);
		assert_eq!(Json::from_str(""), None);
	}



	/* CUSTOM TAG SET */

	fn custom_tags() -> JsonTags {
		JsonTags {
			array_tags: JsonArrayTags::new("{{::", "|_|", "::}}"),
			..Default::default()
		}
	}
	
	#[test]
	fn custom_tags_can_parse_array() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("{{::::}}", &tags), Some(Json::new(Vec::<u8>::new())));
		assert_eq!(Json::from_str_with_tag_set("{{::0|_|2|_|4|_|8::}}", &tags), Some(Json::new(vec![0, 2, 4, 8])));
		assert_eq!(Json::from_str_with_tag_set("{{::0|_|{{::1|_|2|_|3::}}|_|true::}}", &tags), Some(Json::new(vec![Json::new(0), Json::new(vec![1, 2, 3]), Json::new(true)])));
	}
	
	#[test]
	fn custom_tags_can_parse_array_with_whitespace() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("\n\t {{::\n\t ::}}\n\t ", &tags), Some(Json::new(Vec::<u8>::new())));
		assert_eq!(Json::from_str_with_tag_set("\n\t {{::0|_| 2|_| 4|_| 8::}}\n\t ", &tags), Some(Json::new(vec![0, 2, 4, 8])));
		assert_eq!(Json::from_str_with_tag_set("\n\t {{::0|_| {{::1|_| 2|_| 3::}}|_| true::}}\n\t ", &tags), Some(Json::new(vec![Json::new(0), Json::new(vec![1, 2, 3]), Json::new(true)])));
	}

	#[test]
	fn custom_tags_can_not_parse_invalid_array() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("{{::'array_without_end'", &tags), None);
		assert_eq!(Json::from_str_with_tag_set("::}}", &tags), None);
		assert_eq!(Json::from_str_with_tag_set("{{::'broken_sub_array'|_| {{::::}}", &tags), None);
		assert_eq!(Json::from_str_with_tag_set("", &tags), None);
	}
}