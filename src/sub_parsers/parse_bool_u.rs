#[cfg(test)]
mod tests {
	use crate::{ Json, JsonBoolTags, JsonTags };



	/* DEFAULT TAG SET */
	
	#[test]
	fn can_parse_boolean() {
		assert_eq!(Json::from_str("true").unwrap(), Json::new(true));
		assert_eq!(Json::from_str("false").unwrap(), Json::new(false));
	}

	#[test]
	fn can_parse_boolean_case_insensitive() {
		assert_eq!(Json::from_str("true").unwrap(), Json::new(true));
		assert_eq!(Json::from_str("True").unwrap(), Json::new(true));
		assert_eq!(Json::from_str("TrUe").unwrap(), Json::new(true));
		assert_eq!(Json::from_str("TRUE").unwrap(), Json::new(true));

		assert_eq!(Json::from_str("false").unwrap(), Json::new(false));
		assert_eq!(Json::from_str("False").unwrap(), Json::new(false));
		assert_eq!(Json::from_str("FaLse").unwrap(), Json::new(false));
		assert_eq!(Json::from_str("FALSE").unwrap(), Json::new(false));
	}

	#[test]
	fn can_parse_boolean_flip() {
		assert_eq!(Json::from_str("!true").unwrap(), Json::new(false));
		assert_eq!(Json::from_str("!false").unwrap(), Json::new(true));
		assert_eq!(Json::from_str("!!true").unwrap(), Json::new(true));
		assert_eq!(Json::from_str("!!false").unwrap(), Json::new(false));
		assert_eq!(Json::from_str("!!!!!!!!!true").unwrap(), Json::new(false));
		assert_eq!(Json::from_str("!!!!!!!!!false").unwrap(), Json::new(true));
	}

	#[test]
	fn can_parse_boolean_with_whitespace() {
		assert_eq!(Json::from_str("\n\t true\n\t ").unwrap(), Json::new(true));
		assert_eq!(Json::from_str("\n\t false\n\t ").unwrap(), Json::new(false));
		assert_eq!(Json::from_str("\n\t !false\n\t ").unwrap(), Json::new(true));
	}

	#[test]
	fn can_not_parse_invalid_boolean() {
		assert!(Json::from_str("yes").is_err());
		assert!(Json::from_str("nah").is_err());
		assert!(Json::from_str("tr ue").is_err());
		assert!(Json::from_str("fa lse").is_err());
		assert!(Json::from_str("").is_err());
	}

	#[test]
	fn can_convert_to_and_from_json() {
		let original:bool = true;
		let as_json:Json = Json::new(original);
		assert_eq!(original, bool::try_from(as_json).unwrap());
	}

	#[test]
	fn can_convert_to_and_from_json() {
		let original:bool = true;
		let as_json:Json = Json::new(original);
		assert_eq!(original, bool::try_from(as_json).unwrap());
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
		assert_eq!(Json::from_str_with_tag_set("yes", &tags).unwrap(), Json::new(true));
		assert_eq!(Json::from_str_with_tag_set("nah", &tags).unwrap(), Json::new(false));
	}

	#[test]
	fn custom_tags_can_parse_boolean_case_insensitive() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("yes", &tags).unwrap(), Json::new(true));
		assert_eq!(Json::from_str_with_tag_set("Yes", &tags).unwrap(), Json::new(true));
		assert_eq!(Json::from_str_with_tag_set("YeS", &tags).unwrap(), Json::new(true));
		assert_eq!(Json::from_str_with_tag_set("YES", &tags).unwrap(), Json::new(true));

		assert_eq!(Json::from_str_with_tag_set("nah", &tags).unwrap(), Json::new(false));
		assert_eq!(Json::from_str_with_tag_set("Nah", &tags).unwrap(), Json::new(false));
		assert_eq!(Json::from_str_with_tag_set("NaH", &tags).unwrap(), Json::new(false));
		assert_eq!(Json::from_str_with_tag_set("NAH", &tags).unwrap(), Json::new(false));
	}

	#[test]
	fn custom_tags_can_parse_boolean_flip() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("notyes", &tags).unwrap(), Json::new(false));
		assert_eq!(Json::from_str_with_tag_set("notnah", &tags).unwrap(), Json::new(true));
		assert_eq!(Json::from_str_with_tag_set("notnotyes", &tags).unwrap(), Json::new(true));
		assert_eq!(Json::from_str_with_tag_set("notnotnah", &tags).unwrap(), Json::new(false));
		assert_eq!(Json::from_str_with_tag_set("notnotnotnotnotnotnotnotnotyes", &tags).unwrap(), Json::new(false));
		assert_eq!(Json::from_str_with_tag_set("notnotnotnotnotnotnotnotnotnah", &tags).unwrap(), Json::new(true));
	}

	#[test]
	fn custom_tags_can_parse_boolean_with_whitespace() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("\n\t yes\n\t ", &tags).unwrap(), Json::new(true));
		assert_eq!(Json::from_str_with_tag_set("\n\t nah\n\t ", &tags).unwrap(), Json::new(false));
		assert_eq!(Json::from_str_with_tag_set("\n\t notnah\n\t ", &tags).unwrap(), Json::new(true));
	}

	#[test]
	fn custom_tags_can_not_parse_invalid_boolean() {
		let tags:JsonTags = custom_tags();
		assert!(Json::from_str_with_tag_set("true", &tags).is_err());
		assert!(Json::from_str_with_tag_set("false", &tags).is_err());
		assert!(Json::from_str_with_tag_set("y es", &tags).is_err());
		assert!(Json::from_str_with_tag_set("n ah", &tags).is_err());
		assert!(Json::from_str_with_tag_set("", &tags).is_err());
	}
}