use super::Expression;
use crate::data_type::DataType;

pub enum BinaryOperator {
    Plus,  // +
    Minus, // -
    Star,  // *
    Slash, // /

    VerticalBar, // |
    Caret,       // ^
    Ampersand,   // &
}

pub struct BinaryExpression {
    pub operator: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl BinaryExpression {
    pub fn get_data_type(&self) -> DataType {
        match self.operator {
            BinaryOperator::Minus => return DataType::Float,
            BinaryOperator::Star => return DataType::Float,
            BinaryOperator::Slash => return DataType::Float,

            BinaryOperator::VerticalBar => return DataType::Float,
            BinaryOperator::Caret => return DataType::Float,
            BinaryOperator::Ampersand => return DataType::Float,

            BinaryOperator::Plus => {
                let left_data_type = self.left.get_data_type();
                let right_data_type = self.right.get_data_type();

                if left_data_type == DataType::String || right_data_type == DataType::String {
                    return DataType::String;
                } else {
                    return DataType::Float;
                }
            }
        }
    }
}

#[cfg(test)]
mod binary_expression_test {
    use crate::{
        data_type::DataType,
        expression::{literal_expression::LiteralExpression, Expression},
    };

    use super::{BinaryExpression, BinaryOperator};

    #[test]
    fn test_minus_op() {
        let minus_binary_exp = BinaryExpression {
            operator: BinaryOperator::Minus,
            left: Box::new(Expression::Literal(LiteralExpression::Float {
                name: "12".to_string(),
                value: 12.0,
            })),
            right: Box::new(Expression::Literal(LiteralExpression::Float {
                name: "12".to_string(),
                value: 12.0,
            })),
        };

        let expected_data_type = DataType::Float;

        let actual_data_type = minus_binary_exp.get_data_type();

        assert_eq!(expected_data_type, actual_data_type);
    }

    #[test]
    fn test_star_op() {
        let star = BinaryExpression {
            operator: BinaryOperator::Star,
            left: Box::new(Expression::Literal(LiteralExpression::Float {
                name: "12".to_string(),
                value: 12.0,
            })),
            right: Box::new(Expression::Literal(LiteralExpression::Float {
                name: "12".to_string(),
                value: 12.0,
            })),
        };

        let expected_data_type = DataType::Float;

        let actual_data_type = star.get_data_type();

        assert_eq!(expected_data_type, actual_data_type);
    }

    #[test]
    fn test_slash_op() {
        let star_binary_exp = BinaryExpression {
            operator: BinaryOperator::Star,
            left: Box::new(Expression::Literal(LiteralExpression::Float {
                name: "12".to_string(),
                value: 12.0,
            })),
            right: Box::new(Expression::Literal(LiteralExpression::Float {
                name: "12".to_string(),
                value: 12.0,
            })),
        };

        let expected_data_type = DataType::Float;

        let actual_data_type = star_binary_exp.get_data_type();

        assert_eq!(expected_data_type, actual_data_type);
    }

    #[test]
    fn test_vertical_bar_op() {
        let vertical_bar_binary_exp = BinaryExpression {
            operator: BinaryOperator::VerticalBar,
            left: Box::new(Expression::Literal(LiteralExpression::Float {
                name: "12".to_string(),
                value: 12.0,
            })),
            right: Box::new(Expression::Literal(LiteralExpression::Float {
                name: "12".to_string(),
                value: 12.0,
            })),
        };

        let expected_data_type = DataType::Float;

        let actual_data_type = vertical_bar_binary_exp.get_data_type();

        assert_eq!(expected_data_type, actual_data_type);
    }

    #[test]
    fn test_caret_op() {
        let caret_binary_exp = BinaryExpression {
            operator: BinaryOperator::Caret,
            left: Box::new(Expression::Literal(LiteralExpression::Float {
                name: "12".to_string(),
                value: 12.0,
            })),
            right: Box::new(Expression::Literal(LiteralExpression::Float {
                name: "12".to_string(),
                value: 12.0,
            })),
        };

        let expected_data_type = DataType::Float;

        let actual_data_type = caret_binary_exp.get_data_type();

        assert_eq!(expected_data_type, actual_data_type);
    }

    #[test]
    fn test_ampersand_op() {
        let ampersand_binary_exp = BinaryExpression {
            operator: BinaryOperator::Ampersand,
            left: Box::new(Expression::Literal(LiteralExpression::Float {
                name: "12".to_string(),
                value: 12.0,
            })),
            right: Box::new(Expression::Literal(LiteralExpression::Float {
                name: "12".to_string(),
                value: 12.0,
            })),
        };

        let expected_data_type = DataType::Float;

        let actual_data_type = ampersand_binary_exp.get_data_type();

        assert_eq!(expected_data_type, actual_data_type);
    }

    #[test]

    fn test_plus_string_op() {
        let plus_string_binary_exp = BinaryExpression {
            operator: BinaryOperator::Plus,
            left: Box::new(Expression::Literal(LiteralExpression::String {
                name: "12".to_string(),
            })),
            right: Box::new(Expression::Literal(LiteralExpression::String {
                name: "12".to_string(),
            })),
        };

        let expected_data_type = DataType::String;

        let actual_data_type = plus_string_binary_exp.get_data_type();

        assert_eq!(expected_data_type, actual_data_type);
    }

    #[test]
    fn test_plus_float_op() {
        let plus_float_binary_exp = BinaryExpression {
            operator: BinaryOperator::Plus,
            left: Box::new(Expression::Literal(LiteralExpression::Float {
                name: "12".to_string(),
                value: 12.0,
            })),
            right: Box::new(Expression::Literal(LiteralExpression::Boolean {
                name: "true".to_string(),
                value: true,
            })),
        };

        let expected_data_type = DataType::Float;

        let actual_data_type = plus_float_binary_exp.get_data_type();

        assert_eq!(expected_data_type, actual_data_type);
    }
}
