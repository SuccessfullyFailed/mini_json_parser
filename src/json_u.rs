#[cfg(test)]
mod tests {
	use crate::Json;

	

	const JSON_STRING:&str = "{ 'name': 'jeffrey', 'has_int_list': true, 'int_list': [0, 2, 4, 8, 16], random_factor: 19.12 }";



	#[test]
	fn can_parse_generic_json() {
		let json:Json = Json::from_str(JSON_STRING).unwrap();
		assert_eq!(
			json,
			Json::Dict(vec![
				(
					Json::String("name".to_string()),
					Some(
						Json::String("jeffrey".to_string())
					)
				), (
					Json::String("has_int_list".to_string()),
					Some(
						Json::Bool(true)
					)
				), (
					Json::String("int_list".to_string()),
					Some(
						Json::Array(vec![
							Json::Int(0),
							Json::Int(2),
							Json::Int(4),
							Json::Int(8),
							Json::Int(16)
						])
					)
				), (
					Json::String("random_factor".to_string()),
					Some(
						Json::Float(19.12)
					)
				),
			])
		)
	}

	#[test]
	fn can_get_children() {
		let json:Json = Json::from_str(JSON_STRING).unwrap();
		assert_eq!(json.get("name"), Some(&"jeffrey".to_string()));
		assert_eq!(json.get(vec![Json::new("int_list"), Json::new(2)]), Some(&4));
	}

	#[test]
	fn can_get_children_mutable() {
		let mut json:Json = Json::from_str(JSON_STRING).unwrap();
		*json.get_mut::<'_, i64, _>(vec![Json::new("int_list"), Json::new(2)]).unwrap() = 512_i64;
		assert_eq!(json.get(vec![Json::new("int_list"), Json::new(2)]), Some(&512));
	}
}