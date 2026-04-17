#[cfg(test)]
mod tests {
	use crate::{ Json, JsonBoolTags, JsonTags };



	/* DEFAULT TAG SET */
	
	#[test]
	fn can_parse_boolean() {
		assert_eq!(Json::from_str("true"), Some(Json::new(true)));
		assert_eq!(Json::from_str("false"), Some(Json::new(false)));
	}

	#[test]
	fn can_parse_boolean_case_insensitive() {
		assert_eq!(Json::from_str("true"), Some(Json::new(true)));
		assert_eq!(Json::from_str("True"), Some(Json::new(true)));
		assert_eq!(Json::from_str("TrUe"), Some(Json::new(true)));
		assert_eq!(Json::from_str("TRUE"), Some(Json::new(true)));

		assert_eq!(Json::from_str("false"), Some(Json::new(false)));
		assert_eq!(Json::from_str("False"), Some(Json::new(false)));
		assert_eq!(Json::from_str("FaLse"), Some(Json::new(false)));
		assert_eq!(Json::from_str("FALSE"), Some(Json::new(false)));
	}

	#[test]
	fn can_parse_boolean_flip() {
		assert_eq!(Json::from_str("!true"), Some(Json::new(false)));
		assert_eq!(Json::from_str("!false"), Some(Json::new(true)));
		assert_eq!(Json::from_str("!!true"), Some(Json::new(true)));
		assert_eq!(Json::from_str("!!false"), Some(Json::new(false)));
		assert_eq!(Json::from_str("!!!!!!!!!true"), Some(Json::new(false)));
		assert_eq!(Json::from_str("!!!!!!!!!false"), Some(Json::new(true)));
	}

	#[test]
	fn can_parse_boolean_with_whitespace() {
		assert_eq!(Json::from_str("\n\t true\n\t "), Some(Json::new(true)));
		assert_eq!(Json::from_str("\n\t false\n\t "), Some(Json::new(false)));
		assert_eq!(Json::from_str("\n\t !false\n\t "), Some(Json::new(true)));
	}

	#[test]
	fn can_not_parse_invalid_boolean() {
		assert_eq!(Json::from_str("yes"), None);
		assert_eq!(Json::from_str("nah"), None);
		assert_eq!(Json::from_str("tr ue"), None);
		assert_eq!(Json::from_str("fa lse"), None);
		assert_eq!(Json::from_str(""), None);
	}



	/* CUSTOM TAG SET */

	fn custom_tags() -> JsonTags {
		JsonTags {
			bool_tags: JsonBoolTags::new("yes", "nah", "not"), // Test with custom tags should always have tags with different sizes than the default
			..Default::default()
		}
	}
	
	#[test]
	fn custom_tags_can_parse_boolean() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("yes", &tags), Some(Json::new(true)));
		assert_eq!(Json::from_str_with_tag_set("nah", &tags), Some(Json::new(false)));
	}

	#[test]
	fn custom_tags_can_parse_boolean_case_insensitive() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("yes", &tags), Some(Json::new(true)));
		assert_eq!(Json::from_str_with_tag_set("Yes", &tags), Some(Json::new(true)));
		assert_eq!(Json::from_str_with_tag_set("YeS", &tags), Some(Json::new(true)));
		assert_eq!(Json::from_str_with_tag_set("YES", &tags), Some(Json::new(true)));

		assert_eq!(Json::from_str_with_tag_set("nah", &tags), Some(Json::new(false)));
		assert_eq!(Json::from_str_with_tag_set("Nah", &tags), Some(Json::new(false)));
		assert_eq!(Json::from_str_with_tag_set("NaH", &tags), Some(Json::new(false)));
		assert_eq!(Json::from_str_with_tag_set("NAH", &tags), Some(Json::new(false)));
	}

	#[test]
	fn custom_tags_can_parse_boolean_flip() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("notyes", &tags), Some(Json::new(false)));
		assert_eq!(Json::from_str_with_tag_set("notnah", &tags), Some(Json::new(true)));
		assert_eq!(Json::from_str_with_tag_set("notnotyes", &tags), Some(Json::new(true)));
		assert_eq!(Json::from_str_with_tag_set("notnotnah", &tags), Some(Json::new(false)));
		assert_eq!(Json::from_str_with_tag_set("notnotnotnotnotnotnotnotnotyes", &tags), Some(Json::new(false)));
		assert_eq!(Json::from_str_with_tag_set("notnotnotnotnotnotnotnotnotnah", &tags), Some(Json::new(true)));
	}

	#[test]
	fn custom_tags_can_parse_boolean_with_whitespace() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("\n\t yes\n\t ", &tags), Some(Json::new(true)));
		assert_eq!(Json::from_str_with_tag_set("\n\t nah\n\t ", &tags), Some(Json::new(false)));
		assert_eq!(Json::from_str_with_tag_set("\n\t notnah\n\t ", &tags), Some(Json::new(true)));
	}

	#[test]
	fn custom_tags_can_not_parse_invalid_boolean() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("true", &tags), None);
		assert_eq!(Json::from_str_with_tag_set("false", &tags), None);
		assert_eq!(Json::from_str_with_tag_set("y es", &tags), None);
		assert_eq!(Json::from_str_with_tag_set("n ah", &tags), None);
		assert_eq!(Json::from_str_with_tag_set("", &tags), None);
	}
}