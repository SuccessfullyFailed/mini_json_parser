#[cfg(test)]
mod tests {
	use crate::Json;



	#[test]
	fn can_parse_boolean() {
		assert_eq!(Json::new("true"), Some(Json::Bool(true)));
		assert_eq!(Json::new("True"), Some(Json::Bool(true)));
		assert_eq!(Json::new("TRUE"), Some(Json::Bool(true)));

		assert_eq!(Json::new("false"), Some(Json::Bool(false)));
		assert_eq!(Json::new("False"), Some(Json::Bool(false)));
		assert_eq!(Json::new("FALSE"), Some(Json::Bool(false)));
		
		assert_eq!(Json::new("!true"), Some(Json::Bool(false)));
		assert_eq!(Json::new("!!true"), Some(Json::Bool(true)));
		assert_eq!(Json::new("!!!!!!!!!true"), Some(Json::Bool(false)));

		assert_eq!(Json::new("\n\t true"), Some(Json::Bool(true)));
	}

	#[test]
	fn can_parse_integer() {
		assert_eq!(Json::new("0"), Some(Json::Int(0)));
		assert_eq!(Json::new("100"), Some(Json::Int(100)));
		assert_eq!(Json::new("-100"), Some(Json::Int(-100)));
		assert_eq!(Json::new("-10-0"), Some(Json::Int(-10)));
		assert_eq!(Json::new("100_000"), Some(Json::Int(100_000)));
		assert_eq!(Json::new("100x000"), Some(Json::Int(100)));

		assert_eq!(Json::new("abc"), None);
		assert_eq!(Json::new("-"), None);
		assert_eq!(Json::new("--100"), Some(Json::Int(100)));
		
		assert_eq!(Json::new("\n\t 0"), Some(Json::Int(0)));
	}

	#[test]
	fn can_parse_float() {
		assert_eq!(Json::new("0.3"), Some(Json::Float(0.3)));
		assert_eq!(Json::new(".3"), Some(Json::Float(0.3)));
		assert_eq!(Json::new("-0.3"), Some(Json::Float(-0.3)));
		assert_eq!(Json::new("-.3"), Some(Json::Float(-0.3)));
		assert_eq!(Json::new("100.3"), Some(Json::Float(100.3)));
		assert_eq!(Json::new("-100.3"), Some(Json::Float(-100.3)));
		assert_eq!(Json::new("-10.3-0.6"), Some(Json::Float(-10.3)));
		assert_eq!(Json::new("100_000.3"), Some(Json::Float(100_000.3)));
		assert_eq!(Json::new("100.3x000.6"), Some(Json::Float(100.3)));

		assert_eq!(Json::new("abc"), None);
		assert_eq!(Json::new("-"), None);
		assert_eq!(Json::new("--100.0"), Some(Json::Float(100.0)));
		assert_eq!(Json::new("-."), None);

		assert_eq!(Json::new("\n\t 0.3"), Some(Json::Float(0.3)));
	}

	#[test]
	fn can_parse_strings() {
		assert_eq!(Json::new("'test_string'"), Some(Json::String("'test_string'".to_string())));
		assert_eq!(Json::new("\"test_string\""), Some(Json::String("\"test_string\"".to_string())));
		assert_eq!(Json::new("'test\\'s_string'"), Some(Json::String("'test\\'s_string'".to_string())));

		assert_eq!(Json::new("\n\t 'test_string'"), Some(Json::String("'test_string'".to_string())));
	}

	#[test]
	fn can_parse_arrays() {
		assert_eq!(Json::new("[]"), Some(Json::Array(vec![])));
		assert_eq!(Json::new("[0]"), Some(Json::Array(vec![Json::Int(0)])));
		assert_eq!(Json::new("[0,1]"), Some(Json::Array(vec![Json::Int(0), Json::Int(1)])));
		assert_eq!(Json::new("[0, 1]"), Some(Json::Array(vec![Json::Int(0), Json::Int(1)])));
		assert_eq!(Json::new("[0, 1,]"), Some(Json::Array(vec![Json::Int(0), Json::Int(1)])));
		assert_eq!(Json::new("[0, 1, true]"), Some(Json::Array(vec![Json::Int(0), Json::Int(1), Json::Bool(true)])));
		assert_eq!(Json::new("[0, 1, 'test']"), Some(Json::Array(vec![Json::Int(0), Json::Int(1), Json::String("'test'".to_string())])));

		assert_eq!(Json::new("\n\t [0,\n\t 1]"), Some(Json::Array(vec![Json::Int(0), Json::Int(1)])));
	}
}