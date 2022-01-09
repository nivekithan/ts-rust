use core::panic;

use ast::{
    data_type::DataType,
    expression::{BinaryOperator, UnaryOperator},
};
use indexmap::IndexMap;
use lexer::token::Token;

pub(crate) fn convert_token_to_unary_operator(token: &Token) -> UnaryOperator {
    let operator = match token {
        Token::Plus => UnaryOperator::Plus,
        Token::Minus => UnaryOperator::Minus,
        Token::Bang => UnaryOperator::Bang,

        tok => panic!("Cannot convert token {:?} to unaryOperator", tok),
    };

    return operator;
}

pub(crate) fn convert_token_to_binary_operator(token: &Token) -> BinaryOperator {
    let operator = match token {
        Token::Plus => BinaryOperator::Plus,
        Token::Minus => BinaryOperator::Minus,
        Token::Slash => BinaryOperator::Slash,
        Token::Star => BinaryOperator::Star,

        Token::VerticalBar => BinaryOperator::VerticalBar,
        Token::Ampersand => BinaryOperator::Ampersand,
        Token::Caret => BinaryOperator::Caret,

        Token::StrictEquality => BinaryOperator::StrictEquality,
        Token::StrictNotEqual => BinaryOperator::StrictNotEqual,

        Token::LessThan => BinaryOperator::LessThan,
        Token::LessThanOrEqual => BinaryOperator::LessThanOrEqual,

        Token::GreaterThan => BinaryOperator::GreaterThan,
        Token::GreaterThanOrEqual => BinaryOperator::GreaterThanOrEqual,

        tok => panic!("Cannot covert token {:?} to binary operator", tok),
    };

    return operator;
}

pub fn convert_index_map_to_vec(index_map: &IndexMap<String, DataType>) -> Vec<DataType> {
    let mut vec_str: Vec<DataType> = Vec::new();

    for (_, data_type) in index_map {
        vec_str.push(data_type.clone());
    }

    return vec_str;
}
