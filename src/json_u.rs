#[cfg(test)]
mod tests {
	use crate::{ Json, JsonArray, JsonArrayTags, JsonBoolTags, JsonDict, JsonDictTags, JsonNumberTags, JsonStringTags, JsonTagsSet };



	#[test]
	fn can_parse_boolean() {
		assert_eq!(Json::from_str("true"), Some(Json::new(true)));
		assert_eq!(Json::from_str("True"), Some(Json::new(true)));
		assert_eq!(Json::from_str("TRUE"), Some(Json::new(true)));

		assert_eq!(Json::from_str("false"), Some(Json::new(false)));
		assert_eq!(Json::from_str("False"), Some(Json::new(false)));
		assert_eq!(Json::from_str("FALSE"), Some(Json::new(false)));
		
		assert_eq!(Json::from_str("!true"), Some(Json::new(false)));
		assert_eq!(Json::from_str("!!true"), Some(Json::new(true)));
		assert_eq!(Json::from_str("!!!!!!!!!true"), Some(Json::new(false)));

		assert_eq!(Json::from_str("\n\t true"), Some(Json::new(true)));
	}

	#[test]
	fn can_parse_integer() {
		assert_eq!(Json::from_str("0"), Some(Json::new(0)));
		assert_eq!(Json::from_str("100"), Some(Json::new(100)));
		assert_eq!(Json::from_str("-100"), Some(Json::new(-100)));
		assert_eq!(Json::from_str("-10-0"), Some(Json::new(-10)));
		assert_eq!(Json::from_str("100_000"), Some(Json::new(100_000)));
		assert_eq!(Json::from_str("100x000"), Some(Json::new(100)));

		assert_eq!(Json::from_str("abc"), None);
		assert_eq!(Json::from_str("-"), None);
		assert_eq!(Json::from_str("--100"), Some(Json::new(100)));
		
		assert_eq!(Json::from_str("\n\t 0"), Some(Json::new(0)));
	}

	#[test]
	fn can_parse_float() {
		assert_eq!(Json::from_str("0.3"), Some(Json::new(0.3)));
		assert_eq!(Json::from_str(".3"), Some(Json::new(0.3)));
		assert_eq!(Json::from_str("-0.3"), Some(Json::new(-0.3)));
		assert_eq!(Json::from_str("-.3"), Some(Json::new(-0.3)));
		assert_eq!(Json::from_str("100.3"), Some(Json::new(100.3)));
		assert_eq!(Json::from_str("-100.3"), Some(Json::new(-100.3)));
		assert_eq!(Json::from_str("-10.3-0.6"), Some(Json::new(-10.3)));
		assert_eq!(Json::from_str("100_000.3"), Some(Json::new(100_000.3)));
		assert_eq!(Json::from_str("100.3x000.6"), Some(Json::new(100.3)));

		assert_eq!(Json::from_str("abc"), None);
		assert_eq!(Json::from_str("-"), None);
		assert_eq!(Json::from_str("--100.0"), Some(Json::new(100.0)));
		assert_eq!(Json::from_str("-."), None);

		assert_eq!(Json::from_str("\n\t 0.3"), Some(Json::new(0.3)));
	}

	#[test]
	fn can_parse_strings() {
		assert_eq!(Json::from_str("'test_string'"), Some(Json::new("test_string".to_string())));
		assert_eq!(Json::from_str("\"test_string\""), Some(Json::new("test_string".to_string())));
		assert_eq!(Json::from_str("'test\\'s_string'"), Some(Json::new("test\\'s_string".to_string())));

		assert_eq!(Json::from_str("\n\t 'test_string'"), Some(Json::new("test_string".to_string())));
	}

	#[test]
	fn can_parse_arrays() {
		assert_eq!(Json::from_str("[]"), Some(Json::new(JsonArray::new(Vec::new()))));
		assert_eq!(Json::from_str("[0]"), Some(Json::new(vec![Json::new(0)])));
		assert_eq!(Json::from_str("[0,1]"), Some(Json::new(vec![Json::new(0), Json::new(1)])));
		assert_eq!(Json::from_str("[0, 1]"), Some(Json::new(vec![Json::new(0), Json::new(1)])));
		assert_eq!(Json::from_str("[0, 1,]"), Some(Json::new(vec![Json::new(0), Json::new(1)])));
		assert_eq!(Json::from_str("[0, 1, true]"), Some(Json::new(vec![Json::new(0), Json::new(1), Json::new(true)])));
		assert_eq!(Json::from_str("[0, 1, 'test']"), Some(Json::new(vec![Json::new(0), Json::new(1), Json::new("test".to_string())])));

		assert_eq!(Json::from_str("\n\t [0,\n\t 1]"), Some(Json::new(vec![Json::new(0), Json::new(1)])));
	}

	#[test]
	fn can_parse_dicts() {
		assert_eq!(Json::from_str("{}"), Some(Json::new(JsonDict::new(vec![]))));
		assert_eq!(Json::from_str("{0}"), Some(Json::new(vec![(Json::new(0), None)])));
		assert_eq!(Json::from_str("{0,1}"), Some(Json::new(vec![(Json::new(0), None), (Json::new(1), None)])));
		assert_eq!(Json::from_str("{0:'a',1:'b'}"), Some(Json::new(vec![(Json::new(0), Some(Json::new("a".to_string()))), (Json::new(1), Some(Json::new("b".to_string())))])));
		assert_eq!(Json::from_str("{0:'a',1:'b',2}"), Some(Json::new(vec![(Json::new(0), Some(Json::new("a".to_string()))), (Json::new(1), Some(Json::new("b".to_string()))), (Json::new(2), None)])));
		
		assert_eq!(Json::from_str("\n\t {\n\t 0\n\t :\n\t 'a'\n\t ,\n\t 1\n\t :\n\t 'b'\n\t }\n\t "), Some(Json::new(vec![(Json::new(0), Some(Json::new("a".to_string()))), (Json::new(1), Some(Json::new("b".to_string())))])));
	}

	#[test]
	fn to_string() {
		assert_eq!(
			Json::new(vec![(Json::new("values".to_string()), Some(Json::new(vec![Json::new(true), Json::new(10), Json::new(0.3)])))]).to_string(),
			"{\"values\":[true,10,0.3]}"
		);
	}

	#[test]
	fn custom_tags() {
		// TODO: Fix problem here
		let tags:JsonTagsSet = JsonTagsSet::new(
			JsonBoolTags::new("yes", "no", "un"),
			JsonNumberTags::new(",", "x", &["@", "#"]),
			JsonStringTags::new(&[("```", "```", &[("#", 1)])]),
			JsonArrayTags::new("[[", "|", "]]"),
			JsonDictTags::new("{{", "=", "|", "}}")
		);
		let json:Json = Json::from_str_with_tag_set("{{```values```=[[yes|no|unyes|unununno|10|0,3|x0,3]]|1@2#3}}", tags).unwrap();
		assert_eq!(
			json,
			Json::new(vec![
				(
					Json::new("values".to_string()),
					Some(Json::new(vec![
						Json::new(true),
						Json::new(false),
						Json::new(false),
						Json::new(true),
						Json::new(10),
						Json::new(0.3),
						Json::new(-0.3)
					]))
				),
				(
					Json::new(123),
					None
				)
			])
		)
	}

	#[test]
	fn modifications() {
		/*
		TODO:
		let mut original = Json::new(vec![(Json::new("values".to_string()), Some(Json::new(vec![Json::new(true), Json::new(10), Json::new(0.3)])))]);
		*original.find_by_key_mut("values").unwrap().find_by_index_mut(2).unwrap() = Json::new(0.6);
		assert_eq!(original, Json::new(vec![(Json::new("values".to_string()), Some(Json::new(vec![Json::new(true), Json::new(10), Json::new(0.6)])))]));
		*/
	}
}