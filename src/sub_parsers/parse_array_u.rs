#[cfg(test)]
mod tests {
	use crate::{ Json, JsonArrayTags, JsonTags };



	/* DEFAULT TAG SET */
	
	#[test]
	fn can_parse_array() {
		assert_eq!(Json::from_str("[]").unwrap(), Json::new(Vec::<u8>::new()));
		assert_eq!(Json::from_str("[0,2,4,8]").unwrap(), Json::new(vec![0, 2, 4, 8]));
		assert_eq!(Json::from_str("[0,[1,2,3],true]").unwrap(), Json::new(vec![Json::new(0), Json::new(vec![1, 2, 3]), Json::new(true)]));
	}
	
	#[test]
	fn can_parse_array_with_whitespace() {
		assert_eq!(Json::from_str("\n\t [\n\t ]\n\t ").unwrap(), Json::new(Vec::<u8>::new()));
		assert_eq!(Json::from_str("\n\t [0, 2, 4, 8]\n\t ").unwrap(), Json::new(vec![0, 2, 4, 8]));
		assert_eq!(Json::from_str("\n\t [0, [1, 2, 3], true]\n\t ").unwrap(), Json::new(vec![Json::new(0), Json::new(vec![1, 2, 3]), Json::new(true)]));
	}

	#[test]
	fn can_not_parse_invalid_array() {
		assert!(Json::from_str("['array_without_end'").is_err());
		assert!(Json::from_str("]").is_err());
		assert!(Json::from_str("['broken_sub_array', []").is_err());
		assert!(Json::from_str("").is_err());
	}

	#[test]
	fn can_convert_to_and_from_json() {
		let original:Vec<u8> = vec![0, 4, 16, 32, 64, 128, 3];
		let as_json:Json = Json::new(original.clone());
		assert_eq!(original, Vec::<u8>::try_from(as_json).unwrap());
	}

	#[test]
	fn can_convert_to_and_from_json() {
		let original:Vec<u8> = vec![0, 4, 16, 32, 64, 128, 3];
		let as_json:Json = Json::new(original.clone());
		assert_eq!(original, Vec::<u8>::try_from(as_json).unwrap());
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
		assert_eq!(Json::from_str_with_tag_set("{{::::}}", &tags).unwrap(), Json::new(Vec::<u8>::new()));
		assert_eq!(Json::from_str_with_tag_set("{{::0|_|2|_|4|_|8::}}", &tags).unwrap(), Json::new(vec![0, 2, 4, 8]));
		assert_eq!(Json::from_str_with_tag_set("{{::0|_|{{::1|_|2|_|3::}}|_|true::}}", &tags).unwrap(), Json::new(vec![Json::new(0), Json::new(vec![1, 2, 3]), Json::new(true)]));
	}
	
	#[test]
	fn custom_tags_can_parse_array_with_whitespace() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("\n\t {{::\n\t ::}}\n\t ", &tags).unwrap(), Json::new(Vec::<u8>::new()));
		assert_eq!(Json::from_str_with_tag_set("\n\t {{::0|_| 2|_| 4|_| 8::}}\n\t ", &tags).unwrap(), Json::new(vec![0, 2, 4, 8]));
		assert_eq!(Json::from_str_with_tag_set("\n\t {{::0|_| {{::1|_| 2|_| 3::}}|_| true::}}\n\t ", &tags).unwrap(), Json::new(vec![Json::new(0), Json::new(vec![1, 2, 3]), Json::new(true)]));
	}

	#[test]
	fn custom_tags_can_not_parse_invalid_array() {
		let tags:JsonTags = custom_tags();
		assert!(Json::from_str_with_tag_set("{{::'array_without_end'", &tags).is_err());
		assert!(Json::from_str_with_tag_set("::}}", &tags).is_err());
		assert!(Json::from_str_with_tag_set("{{::'broken_sub_array'|_| {{::::}}", &tags).is_err());
		assert!(Json::from_str_with_tag_set("", &tags).is_err());
	}
}