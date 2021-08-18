use crate::data_type::DataType;

use super::Expression;

pub enum UnaryOperator {
    Plus,  // +
    Minus, // -
    Bang,  // -
}

pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub argument: Box<Expression>,
}

impl UnaryExpression {
    pub fn get_data_type(&self) -> DataType {
        match self.operator {
            UnaryOperator::Bang => return DataType::Boolean,
            UnaryOperator::Minus => return DataType::Float,
            UnaryOperator::Plus => return DataType::Float,
        }
    }
}

#[cfg(test)]
mod unary_test {
    use crate::{
        data_type::DataType,
        expression::{literal_expression::LiteralExpression, Expression},
    };

    use super::{UnaryExpression, UnaryOperator};

    #[test]
    fn test_plus_op() {
        let plus_unary_exp = UnaryExpression {
            operator: UnaryOperator::Plus,
            argument: Box::new(Expression::Literal(LiteralExpression::Float {
                name: "123".to_string(),
                value: 123.0,
            })),
        };

        let expected_data_type = DataType::Float;

        let actual_data_type = plus_unary_exp.get_data_type();

        assert_eq!(expected_data_type, actual_data_type);
    }

    #[test]
    fn test_minus_op() {
        let minus_unary_exp = UnaryExpression {
            operator: UnaryOperator::Minus,
            argument: Box::new(Expression::Literal(LiteralExpression::Float {
                name: "123".to_string(),
                value: 123.0,
            })),
        };

        let expected_data_type = DataType::Float;

        let actual_data_type = minus_unary_exp.get_data_type();

        assert_eq!(expected_data_type, actual_data_type);
    }

    #[test]
    fn test_bang_op() {
        let bang_unary_exp = UnaryExpression {
            operator: UnaryOperator::Bang,
            argument: Box::new(Expression::Literal(LiteralExpression::Float {
                name: "123".to_string(),
                value: 123.0,
            })),
        };

        let expected_data_type = DataType::Boolean;

        let actual_data_type = bang_unary_exp.get_data_type();

        assert_eq!(expected_data_type, actual_data_type);
    }
}
