#[cfg(test)]
mod tests {
	use crate::{ Json, JsonNumberTags, JsonTagsSet };



	/* DEFAULT TAG SET */
	
	#[test]
	fn can_parse_int() {
		assert_eq!(Json::from_str("0"), Some(Json::new(0)));
		assert_eq!(Json::from_str("12"), Some(Json::new(12)));
		assert_eq!(Json::from_str("64000"), Some(Json::new(64000)));
	}
	
	#[test]
	fn can_parse_float() {
		assert_eq!(Json::from_str("0.0"), Some(Json::new(0.0)));
		assert_eq!(Json::from_str("12.8"), Some(Json::new(12.8)));
		assert_eq!(Json::from_str("64000.6"), Some(Json::new(64000.6)));
	}
	
	#[test]
	fn can_parse_int_negative() {
		assert_eq!(Json::from_str("-0"), Some(Json::new(-0)));
		assert_eq!(Json::from_str("-12"), Some(Json::new(-12)));
		assert_eq!(Json::from_str("-64000"), Some(Json::new(-64000)));
	}
	
	#[test]
	fn can_parse_float_negative() {
		assert_eq!(Json::from_str("-0.0"), Some(Json::new(-0.0)));
		assert_eq!(Json::from_str("-12.8"), Some(Json::new(-12.8)));
		assert_eq!(Json::from_str("-64000.6"), Some(Json::new(-64000.6)));
	}
	
	#[test]
	fn can_parse_int_stacking_negative() {
		assert_eq!(Json::from_str("--0"), Some(Json::new(0)));
		assert_eq!(Json::from_str("---12"), Some(Json::new(-12)));
		assert_eq!(Json::from_str("----64000"), Some(Json::new(64000)));
	}
	
	#[test]
	fn can_parse_float_stacking_negative() {
		assert_eq!(Json::from_str("--0.0"), Some(Json::new(0.0)));
		assert_eq!(Json::from_str("---12.8"), Some(Json::new(-12.8)));
		assert_eq!(Json::from_str("----64000.6"), Some(Json::new(64000.6)));
	}
	
	#[test]
	fn can_parse_int_with_decoration() {
		assert_eq!(Json::from_str("_0"), Some(Json::new(0)));
		assert_eq!(Json::from_str("12_"), Some(Json::new(12)));
		assert_eq!(Json::from_str("64_000"), Some(Json::new(64000)));
	}
	
	#[test]
	fn can_parse_float_with_decoration() {
		assert_eq!(Json::from_str("_0.0"), Some(Json::new(0.0)));
		assert_eq!(Json::from_str("12.8_"), Some(Json::new(12.8)));
		assert_eq!(Json::from_str("64_000_._6"), Some(Json::new(64000.6)));
	}

	#[test]
	fn can_parse_int_with_whitespace() {
		assert_eq!(Json::from_str("\n\t 0\n\t "), Some(Json::new(0)));
		assert_eq!(Json::from_str("\n\t 12\n\t "), Some(Json::new(12)));
		assert_eq!(Json::from_str("\n\t 64000\n\t "), Some(Json::new(64000)));
	}
	
	#[test]
	fn can_parse_float_with_whitespace() {
		assert_eq!(Json::from_str("\n\t 0.0\n\t "), Some(Json::new(0.0)));
		assert_eq!(Json::from_str("\n\t 12.8\n\t "), Some(Json::new(12.8)));
		assert_eq!(Json::from_str("\n\t 64000.6\n\t "), Some(Json::new(64000.6)));
	}

	#[test]
	fn can_not_parse_invalid_int() {
		assert_eq!(Json::from_str("b0"), None);
		assert_eq!(Json::from_str("-."), None);
		assert_eq!(Json::from_str(""), None);
	}



	/* CUSTOM TAG SET */

	fn custom_tags() -> JsonTagsSet {
		JsonTagsSet {
			number_tags: JsonNumberTags::new("::", "!!", &["@@", "##"]), // Test with custom tags should always have tags with different sizes than the default
			..Default::default()
		}
	}
	
	#[test]
	fn custom_tags_can_parse_int() {
		let tags:JsonTagsSet = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("0", tags.clone()), Some(Json::new(0)));
		assert_eq!(Json::from_str_with_tag_set("12", tags.clone()), Some(Json::new(12)));
		assert_eq!(Json::from_str_with_tag_set("64000", tags.clone()), Some(Json::new(64000)));
	}
	
	#[test]
	fn custom_tags_can_parse_float() {
		let tags:JsonTagsSet = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("0::0", tags.clone()), Some(Json::new(0.0)));
		assert_eq!(Json::from_str_with_tag_set("12::8", tags.clone()), Some(Json::new(12.8)));
		assert_eq!(Json::from_str_with_tag_set("64000::6", tags.clone()), Some(Json::new(64000.6)));
	}
	
	#[test]
	fn custom_tags_can_parse_int_negative() {
		let tags:JsonTagsSet = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("!!0", tags.clone()), Some(Json::new(-0)));
		assert_eq!(Json::from_str_with_tag_set("!!12", tags.clone()), Some(Json::new(-12)));
		assert_eq!(Json::from_str_with_tag_set("!!64000", tags.clone()), Some(Json::new(-64000)));
	}
	
	#[test]
	fn custom_tags_can_parse_float_negative() {
		let tags:JsonTagsSet = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("!!0::0", tags.clone()), Some(Json::new(-0.0)));
		assert_eq!(Json::from_str_with_tag_set("!!12::8", tags.clone()), Some(Json::new(-12.8)));
		assert_eq!(Json::from_str_with_tag_set("!!64000::6", tags.clone()), Some(Json::new(-64000.6)));
	}
	
	#[test]
	fn custom_tags_can_parse_int_stacking_negative() {
		let tags:JsonTagsSet = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("!!!!0", tags.clone()), Some(Json::new(0)));
		assert_eq!(Json::from_str_with_tag_set("!!!!!!12", tags.clone()), Some(Json::new(-12)));
		assert_eq!(Json::from_str_with_tag_set("!!!!!!!!64000", tags.clone()), Some(Json::new(64000)));
	}
	
	#[test]
	fn custom_tags_can_parse_float_stacking_negative() {
		let tags:JsonTagsSet = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("!!!!0::0", tags.clone()), Some(Json::new(0.0)));
		assert_eq!(Json::from_str_with_tag_set("!!!!!!12::8", tags.clone()), Some(Json::new(-12.8)));
		assert_eq!(Json::from_str_with_tag_set("!!!!!!!!64000::6", tags.clone()), Some(Json::new(64000.6)));
	}
	
	#[test]
	fn custom_tags_can_parse_int_with_decoration() {
		let tags:JsonTagsSet = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("@@0", tags.clone()), Some(Json::new(0)));
		assert_eq!(Json::from_str_with_tag_set("12##", tags.clone()), Some(Json::new(12)));
		assert_eq!(Json::from_str_with_tag_set("64@@000", tags.clone()), Some(Json::new(64000)));
	}
	
	#[test]
	fn custom_tags_can_parse_float_with_decoration() {
		let tags:JsonTagsSet = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("@@0::0", tags.clone()), Some(Json::new(0.0)));
		assert_eq!(Json::from_str_with_tag_set("12::8##", tags.clone()), Some(Json::new(12.8)));
		assert_eq!(Json::from_str_with_tag_set("64@@000##::@@6", tags.clone()), Some(Json::new(64000.6)));
	}

	#[test]
	fn custom_tags_can_parse_int_with_whitespace() {
		let tags:JsonTagsSet = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("\n\t 0\n\t ", tags.clone()), Some(Json::new(0)));
		assert_eq!(Json::from_str_with_tag_set("\n\t 12\n\t ", tags.clone()), Some(Json::new(12)));
		assert_eq!(Json::from_str_with_tag_set("\n\t 64000\n\t ", tags.clone()), Some(Json::new(64000)));
	}
	
	#[test]
	fn custom_tags_can_parse_float_with_whitespace() {
		let tags:JsonTagsSet = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("\n\t 0::0\n\t ", tags.clone()), Some(Json::new(0.0)));
		assert_eq!(Json::from_str_with_tag_set("\n\t 12::8\n\t ", tags.clone()), Some(Json::new(12.8)));
		assert_eq!(Json::from_str_with_tag_set("\n\t 64000::6\n\t ", tags.clone()), Some(Json::new(64000.6)));
	}

	#[test]
	fn custom_tags_can_not_parse_invalid_int() {
		let tags:JsonTagsSet = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("b0", tags.clone()), None);
		assert_eq!(Json::from_str_with_tag_set("!!::", tags.clone()), None);
		assert_eq!(Json::from_str_with_tag_set("", tags.clone()), None);
	}
}