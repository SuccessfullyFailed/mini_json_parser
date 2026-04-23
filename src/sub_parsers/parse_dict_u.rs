#[cfg(test)]
mod tests {
	use crate::{ Json, JsonDictTags, JsonTags };



	/* DEFAULT TAG SET */
	
	#[test]
	fn can_parse_dict() {
		assert_eq!(Json::from_str("{}"), Some(Json::new(Vec::<(u8, Option<u8>)>::new())));
		assert_eq!(Json::from_str("{0,2,4,8}"), Some(Json::new(vec![(0, None::<u8>), (2, None::<u8>), (4, None::<u8>), (8, None::<u8>)])));
		assert_eq!(Json::from_str("{0:'a',2:'b',4:'c',8:'d'}"), Some(Json::new(vec![(0, Some("a")), (2, Some("b")), (4, Some("c")), (8, Some("d"))])));
	}
	
	#[test]
	fn can_parse_dict_with_whitespace() {
		assert_eq!(Json::from_str("\n\t {}\n\t "), Some(Json::new(Vec::<(u8, Option<u8>)>::new())));
		assert_eq!(Json::from_str("\n\t { 0, 2, 4, 8 }\n\t "), Some(Json::new(vec![(0, None::<u8>), (2, None::<u8>), (4, None::<u8>), (8, None::<u8>)])));
		assert_eq!(Json::from_str("\n\t { 0: 'a', 2: 'b', 4: 'c', 8: 'd' }\n\t "), Some(Json::new(vec![(0, Some("a")), (2, Some("b")), (4, Some("c")), (8, Some("d"))])));
	}

	#[test]
	fn can_not_parse_invalid_dict() {
		assert_eq!(Json::from_str("{'dict_without_end'"), None);
		assert_eq!(Json::from_str("}"), None);
		assert_eq!(Json::from_str("{'broken_sub_dict', {}"), None);
		assert_eq!(Json::from_str(""), None);
	}

	#[test]
	fn can_convert_to_and_from_json() {
		let original:Vec<(u8, Option<bool>)> = vec![(0, None), (4, Some(true)), (16, Some(false)), (32, None)];
		let as_json:Json = Json::new(original.clone());
		assert_eq!(original, Vec::<(u8, Option<bool>)>::try_from(as_json).unwrap());
	}



	/* CUSTOM TAG SET */

	fn custom_tags() -> JsonTags {
		JsonTags {
			dict_tags: JsonDictTags::new("{{::", "==", "|_|", "::}}"),
			..Default::default()
		}
	}
	
	#[test]
	fn custom_tags_can_parse_dict() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("{{::::}}", &tags), Some(Json::new(Vec::<(u8, Option<u8>)>::new())));
		assert_eq!(Json::from_str_with_tag_set("{{::0|_|2|_|4|_|8::}}", &tags), Some(Json::new(vec![(0, None::<u8>), (2, None::<u8>), (4, None::<u8>), (8, None::<u8>)])));
		assert_eq!(Json::from_str_with_tag_set("{{::0=='a'|_|2=='b'|_|4=='c'|_|8=='d'::}}", &tags), Some(Json::new(vec![(0, Some("a")), (2, Some("b")), (4, Some("c")), (8, Some("d"))])));
	}
	
	#[test]
	fn custom_tags_can_parse_dict_with_whitespace() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("\n\t {{::::}}\n\t ", &tags), Some(Json::new(Vec::<(u8, Option<u8>)>::new())));
		assert_eq!(Json::from_str_with_tag_set("\n\t {{:: 0|_| 2|_| 4|_| 8 ::}}\n\t ", &tags), Some(Json::new(vec![(0, None::<u8>), (2, None::<u8>), (4, None::<u8>), (8, None::<u8>)])));
		assert_eq!(Json::from_str_with_tag_set("\n\t {{:: 0 == 'a'|_| 2 == 'b'|_| 4 == 'c'|_| 8 == 'd' ::}}\n\t ", &tags), Some(Json::new(vec![(0, Some("a")), (2, Some("b")), (4, Some("c")), (8, Some("d"))])));
	}

	#[test]
	fn custom_tags_can_not_parse_invalid_dict() {
		let tags:JsonTags = custom_tags();
		assert_eq!(Json::from_str_with_tag_set("{{::'dict_without_end'", &tags), None);
		assert_eq!(Json::from_str_with_tag_set("::}}", &tags), None);
		assert_eq!(Json::from_str_with_tag_set("{{::'broken_sub_dict'|_| {{::::}}", &tags), None);
		assert_eq!(Json::from_str_with_tag_set("", &tags), None);
	}
}