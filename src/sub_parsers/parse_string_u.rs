#[cfg(test)]
mod tests {
	use crate::{ Json, JsonStringTags, JsonTags };



	/* DEFAULT TAG SET */
	
	#[test]
	fn can_parse_string() {
		assert_eq!(Json::from_str("'test'"), Some(Json::new("test")));
		assert_eq!(Json::from_str("\"test\""), Some(Json::new("test")));
		assert_eq!(Json::from_str("\"test with spaces\""), Some(Json::new("test with spaces")));
	}
	
	#[test]
	fn can_parse_escaped_string() {
		assert_eq!(Json::from_str("'te\\'st'"), Some(Json::new("te'st")));
		assert_eq!(Json::from_str("\"te\\\"st\""), Some(Json::new("te\"st")));
		assert_eq!(Json::from_str("\"test \\\"with\\\" spaces\""), Some(Json::new("test \"with\" spaces")));
	}
	
	#[test]
	fn can_parse_string_with_whitespace() {
		assert_eq!(Json::from_str("\n\t '\n\t test\n\t '\n\t "), Some(Json::new("\n\t test\n\t ")));
		assert_eq!(Json::from_str("\n\t \"\n\t test\n\t \"\n\t "), Some(Json::new("\n\t test\n\t ")));
		assert_eq!(Json::from_str("\n\t \"\n\t test with spaces\n\t \"\n\t "), Some(Json::new("\n\t test with spaces\n\t ")));
	}

	#[test]
	fn can_not_parse_invalid_string() {
		assert_eq!(Json::from_str("test_without_quotes"), None);
		assert_eq!(Json::from_str("\"test with one quote"), None);
		assert_eq!(Json::from_str(""), None);
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
		assert_eq!(Json::from_str_with_tag_set("{{::test::}}", &tags), Some(Json::new("test")));
		assert_eq!(Json::from_str_with_tag_set("{{::test with spaces::}}", &tags), Some(Json::new("test with spaces")));
	}
	
	#[test]
	fn custom_tags_can_parse_escaped_string() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("{{::test//12::}}78::}}'", &tags), Some(Json::new("test12::}}78")));
	}
	
	#[test]
	fn custom_tags_can_parse_string_with_whitespace() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("\n\t {{::\n\t test\n\t ::}}\n\t ", &tags), Some(Json::new("\n\t test\n\t ")));
		assert_eq!(Json::from_str_with_tag_set("\n\t {{::\n\t test with spaces\n\t ::}}\n\t ", &tags), Some(Json::new("\n\t test with spaces\n\t ")));
	}

	#[test]
	fn custom_tags_can_not_parse_invalid_string() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("test_without_quotes", &tags), None);
		assert_eq!(Json::from_str_with_tag_set("\"test with one quote", &tags), None);
		assert_eq!(Json::from_str_with_tag_set("\"test with original quotes\"", &tags), None);
		assert_eq!(Json::from_str_with_tag_set("", &tags), None);
	}
}