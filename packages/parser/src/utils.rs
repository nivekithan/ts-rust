use core::panic;

use ast::expression::{BinaryOperator, UnaryOperator};
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
