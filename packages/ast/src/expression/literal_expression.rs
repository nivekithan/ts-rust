use crate::data_type::DataType;

pub enum LiteralExpression {
    Float { name: String, value: f64 },
    String { name: String },
    Boolean { name: String, value: bool },
}

impl LiteralExpression {
    pub fn get_data_type(&self) -> DataType {
        match self {
            &LiteralExpression::Float { name: _, value: _ } => return DataType::Float,
            &LiteralExpression::Boolean { name: _, value: _ } => return DataType::Boolean,
            &LiteralExpression::String { name: _ } => return DataType::String,
        }
    }
}
#[cfg(test)]
mod expression_test {
    use crate::{data_type::DataType, expression::literal_expression::LiteralExpression};

    #[test]
    fn test_float_literal_exp() {
        let float_literal_exp = LiteralExpression::Float {
            name: String::from("123"),
            value: 123.0,
        };

        let expected_data_type = DataType::Float;

        let actual_data_type = float_literal_exp.get_data_type();

        assert_eq!(expected_data_type, actual_data_type);
    }

    #[test]

    fn test_boolean_literal_exp() {
        let boolean_literal_exp = LiteralExpression::Boolean {
            name: "true".to_string(),
            value: true,
        };

        let expected_data_type = DataType::Boolean;

        let actual_data_type = boolean_literal_exp.get_data_type();

        assert_eq!(expected_data_type, actual_data_type);
    }

    #[test]
    fn test_string_literal_exp() {
        let string_literal_exp = LiteralExpression::String {
            name: "12".to_string(),
        };

        let expected_data_type = DataType::String;

        let actual_data_type = string_literal_exp.get_data_type();

        assert_eq!(expected_data_type, actual_data_type);
    }
}
