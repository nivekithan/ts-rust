use crate::data_type::DataType;

use self::{
    binary_expression::BinaryExpression, literal_expression::LiteralExpression,
    unary_expression::UnaryExpression,
};

pub mod binary_expression;
pub mod literal_expression;
pub mod unary_expression;

pub enum Expression {
    Literal(LiteralExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
}

impl Expression {
    pub fn get_data_type(&self) -> DataType {
        match self {
            Expression::Binary(exp) => return exp.get_data_type(),
            Expression::Literal(exp) => return exp.get_data_type(),
            Expression::Unary(exp) => return exp.get_data_type(),
        }
    }
}
