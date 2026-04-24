#[cfg(test)]
mod tests {
	use crate::{ Json, JsonNumberTags, JsonTags };



	/* DEFAULT TAG SET */
	
	#[test]
	fn can_parse_int() {
		assert_eq!(Json::from_str("0").unwrap(), Json::new(0));
		assert_eq!(Json::from_str("12").unwrap(), Json::new(12));
		assert_eq!(Json::from_str("64000").unwrap(), Json::new(64000));
	}
	
	#[test]
	fn can_parse_float() {
		assert_eq!(Json::from_str("0.0").unwrap(), Json::new(0.0));
		assert_eq!(Json::from_str("12.8").unwrap(), Json::new(12.8));
		assert_eq!(Json::from_str("64000.6").unwrap(), Json::new(64000.6));
	}
	
	#[test]
	fn can_parse_int_negative() {
		assert_eq!(Json::from_str("-0").unwrap(), Json::new(-0));
		assert_eq!(Json::from_str("-12").unwrap(), Json::new(-12));
		assert_eq!(Json::from_str("-64000").unwrap(), Json::new(-64000));
	}
	
	#[test]
	fn can_parse_float_negative() {
		assert_eq!(Json::from_str("-0.0").unwrap(), Json::new(-0.0));
		assert_eq!(Json::from_str("-12.8").unwrap(), Json::new(-12.8));
		assert_eq!(Json::from_str("-64000.6").unwrap(), Json::new(-64000.6));
	}
	
	#[test]
	fn can_parse_int_stacking_negative() {
		assert_eq!(Json::from_str("--0").unwrap(), Json::new(0));
		assert_eq!(Json::from_str("---12").unwrap(), Json::new(-12));
		assert_eq!(Json::from_str("----64000").unwrap(), Json::new(64000));
	}
	
	#[test]
	fn can_parse_float_stacking_negative() {
		assert_eq!(Json::from_str("--0.0").unwrap(), Json::new(0.0));
		assert_eq!(Json::from_str("---12.8").unwrap(), Json::new(-12.8));
		assert_eq!(Json::from_str("----64000.6").unwrap(), Json::new(64000.6));
	}
	
	#[test]
	fn can_parse_int_with_decoration() {
		assert_eq!(Json::from_str("_0").unwrap(), Json::new(0));
		assert_eq!(Json::from_str("12_").unwrap(), Json::new(12));
		assert_eq!(Json::from_str("64_000").unwrap(), Json::new(64000));
	}
	
	#[test]
	fn can_parse_float_with_decoration() {
		assert_eq!(Json::from_str("_0.0").unwrap(), Json::new(0.0));
		assert_eq!(Json::from_str("12.8_").unwrap(), Json::new(12.8));
		assert_eq!(Json::from_str("64_000_._6").unwrap(), Json::new(64000.6));
	}

	#[test]
	fn can_parse_int_with_whitespace() {
		assert_eq!(Json::from_str("\n\t 0\n\t ").unwrap(), Json::new(0));
		assert_eq!(Json::from_str("\n\t 12\n\t ").unwrap(), Json::new(12));
		assert_eq!(Json::from_str("\n\t 64000\n\t ").unwrap(), Json::new(64000));
	}
	
	#[test]
	fn can_parse_float_with_whitespace() {
		assert_eq!(Json::from_str("\n\t 0.0\n\t ").unwrap(), Json::new(0.0));
		assert_eq!(Json::from_str("\n\t 12.8\n\t ").unwrap(), Json::new(12.8));
		assert_eq!(Json::from_str("\n\t 64000.6\n\t ").unwrap(), Json::new(64000.6));
	}

	#[test]
	fn can_not_parse_invalid_int() {
		assert!(Json::from_str("b0").is_err());
		assert!(Json::from_str("-.").is_err());
		assert!(Json::from_str("").is_err());
	}

	#[test]
	fn can_convert_to_and_from_json() {
		let original:i64 = -64_912;
		let as_json:Json = Json::new(original);
		assert_eq!(original, i64::try_from(as_json).unwrap());
	}



	/* CUSTOM TAG SET */

	fn custom_tags() -> JsonTags {
		JsonTags {
			number_tags: JsonNumberTags::new("::", "!!", &["@@", "##"]), // Test with custom tags should always have tags with different sizes than the default
			..Default::default()
		}
	}
	
	#[test]
	fn custom_tags_can_parse_int() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("0", &tags).unwrap(), Json::new(0));
		assert_eq!(Json::from_str_with_tag_set("12", &tags).unwrap(), Json::new(12));
		assert_eq!(Json::from_str_with_tag_set("64000", &tags).unwrap(), Json::new(64000));
	}
	
	#[test]
	fn custom_tags_can_parse_float() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("0::0", &tags).unwrap(), Json::new(0.0));
		assert_eq!(Json::from_str_with_tag_set("12::8", &tags).unwrap(), Json::new(12.8));
		assert_eq!(Json::from_str_with_tag_set("64000::6", &tags).unwrap(), Json::new(64000.6));
	}
	
	#[test]
	fn custom_tags_can_parse_int_negative() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("!!0", &tags).unwrap(), Json::new(-0));
		assert_eq!(Json::from_str_with_tag_set("!!12", &tags).unwrap(), Json::new(-12));
		assert_eq!(Json::from_str_with_tag_set("!!64000", &tags).unwrap(), Json::new(-64000));
	}
	
	#[test]
	fn custom_tags_can_parse_float_negative() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("!!0::0", &tags).unwrap(), Json::new(-0.0));
		assert_eq!(Json::from_str_with_tag_set("!!12::8", &tags).unwrap(), Json::new(-12.8));
		assert_eq!(Json::from_str_with_tag_set("!!64000::6", &tags).unwrap(), Json::new(-64000.6));
	}
	
	#[test]
	fn custom_tags_can_parse_int_stacking_negative() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("!!!!0", &tags).unwrap(), Json::new(0));
		assert_eq!(Json::from_str_with_tag_set("!!!!!!12", &tags).unwrap(), Json::new(-12));
		assert_eq!(Json::from_str_with_tag_set("!!!!!!!!64000", &tags).unwrap(), Json::new(64000));
	}
	
	#[test]
	fn custom_tags_can_parse_float_stacking_negative() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("!!!!0::0", &tags).unwrap(), Json::new(0.0));
		assert_eq!(Json::from_str_with_tag_set("!!!!!!12::8", &tags).unwrap(), Json::new(-12.8));
		assert_eq!(Json::from_str_with_tag_set("!!!!!!!!64000::6", &tags).unwrap(), Json::new(64000.6));
	}
	
	#[test]
	fn custom_tags_can_parse_int_with_decoration() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("@@0", &tags).unwrap(), Json::new(0));
		assert_eq!(Json::from_str_with_tag_set("12##", &tags).unwrap(), Json::new(12));
		assert_eq!(Json::from_str_with_tag_set("64@@000", &tags).unwrap(), Json::new(64000));
	}
	
	#[test]
	fn custom_tags_can_parse_float_with_decoration() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("@@0::0", &tags).unwrap(), Json::new(0.0));
		assert_eq!(Json::from_str_with_tag_set("12::8##", &tags).unwrap(), Json::new(12.8));
		assert_eq!(Json::from_str_with_tag_set("64@@000##::@@6", &tags).unwrap(), Json::new(64000.6));
	}

	#[test]
	fn custom_tags_can_parse_int_with_whitespace() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("\n\t 0\n\t ", &tags).unwrap(), Json::new(0));
		assert_eq!(Json::from_str_with_tag_set("\n\t 12\n\t ", &tags).unwrap(), Json::new(12));
		assert_eq!(Json::from_str_with_tag_set("\n\t 64000\n\t ", &tags).unwrap(), Json::new(64000));
	}
	
	#[test]
	fn custom_tags_can_parse_float_with_whitespace() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("\n\t 0::0\n\t ", &tags).unwrap(), Json::new(0.0));
		assert_eq!(Json::from_str_with_tag_set("\n\t 12::8\n\t ", &tags).unwrap(), Json::new(12.8));
		assert_eq!(Json::from_str_with_tag_set("\n\t 64000::6\n\t ", &tags).unwrap(), Json::new(64000.6));
	}

	#[test]
	fn custom_tags_can_not_parse_invalid_int() {
		let tags:JsonTags = custom_tags();
		assert!(Json::from_str_with_tag_set("b0", &tags).is_err());
		assert!(Json::from_str_with_tag_set("!!::", &tags).is_err());
		assert!(Json::from_str_with_tag_set("", &tags).is_err());
	}
}