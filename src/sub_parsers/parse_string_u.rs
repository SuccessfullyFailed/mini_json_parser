#[cfg(test)]
mod tests {
	use crate::{ Json, JsonStringTags, JsonTags };



	/* DEFAULT TAG SET */
	
	#[test]
	fn can_parse_string() {
		assert_eq!(Json::from_str("'test'").unwrap(), Json::new("test"));
		assert_eq!(Json::from_str("\"test\"").unwrap(), Json::new("test"));
		assert_eq!(Json::from_str("\"test with spaces\"").unwrap(), Json::new("test with spaces"));
	}
	
	#[test]
	fn can_parse_escaped_string() {
		assert_eq!(Json::from_str("'te\\'st'").unwrap(), Json::new("te'st"));
		assert_eq!(Json::from_str("\"te\\\"st\"").unwrap(), Json::new("te\"st"));
		assert_eq!(Json::from_str("\"test \\\"with\\\" spaces\"").unwrap(), Json::new("test \"with\" spaces"));
	}
	
	#[test]
	fn can_parse_string_with_whitespace() {
		assert_eq!(Json::from_str("\n\t '\n\t test\n\t '\n\t ").unwrap(), Json::new("\n\t test\n\t "));
		assert_eq!(Json::from_str("\n\t \"\n\t test\n\t \"\n\t ").unwrap(), Json::new("\n\t test\n\t "));
		assert_eq!(Json::from_str("\n\t \"\n\t test with spaces\n\t \"\n\t ").unwrap(), Json::new("\n\t test with spaces\n\t "));
	}

	#[test]
	fn can_not_parse_invalid_string() {
		assert!(Json::from_str("test_without_quotes").is_err());
		assert!(Json::from_str("\"test with one quote").is_err());
		assert!(Json::from_str("").is_err());
	}

	#[test]
	fn can_convert_to_and_from_json() {
		let original:String = String::from("test_string::?");
		let as_json:Json = Json::new(original.clone());
		assert_eq!(original, String::try_from(as_json).unwrap());
	}



	/* CUSTOM TAG SET */

	fn custom_tags() -> JsonTags {
		JsonTags {
			string_tags: JsonStringTags::new(&[("{{::", "::}}", &[("//", 8)])]), // Test with custom tags should always have tags with different sizes than the default
			..Default::default()
		}
	}
	
	#[test]
	fn custom_tags_can_parse_string() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("{{::test::}}", &tags).unwrap(), Json::new("test"));
		assert_eq!(Json::from_str_with_tag_set("{{::test with spaces::}}", &tags).unwrap(), Json::new("test with spaces"));
	}
	
	#[test]
	fn custom_tags_can_parse_escaped_string() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("{{::test//12::}}78::}}'", &tags).unwrap(), Json::new("test12::}}78"));
	}
	
	#[test]
	fn custom_tags_can_parse_string_with_whitespace() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("\n\t {{::\n\t test\n\t ::}}\n\t ", &tags).unwrap(), Json::new("\n\t test\n\t "));
		assert_eq!(Json::from_str_with_tag_set("\n\t {{::\n\t test with spaces\n\t ::}}\n\t ", &tags).unwrap(), Json::new("\n\t test with spaces\n\t "));
	}

	#[test]
	fn custom_tags_can_not_parse_invalid_string() {
		let tags:JsonTags = custom_tags();
		assert!(Json::from_str_with_tag_set("test_without_quotes", &tags).is_err());
		assert!(Json::from_str_with_tag_set("\"test with one quote", &tags).is_err());
		assert!(Json::from_str_with_tag_set("\"test with original quotes\"", &tags).is_err());
		assert!(Json::from_str_with_tag_set("", &tags).is_err());
	}
}