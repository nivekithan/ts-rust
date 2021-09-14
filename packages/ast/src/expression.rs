use lexer::token::Token;

use crate::data_type::DataType;

#[derive(Debug, PartialEq, Clone)]

pub enum UnaryOperator {
    Plus,  // +
    Minus, // -
    Bang,  // !
}
#[derive(Debug, PartialEq, Clone)]

pub enum BinaryOperator {
    Plus,  // +
    Minus, // -
    Star,  // *
    Slash, // /

    VerticalBar, // |
    Caret,       // ^
    Ampersand,   // &

    StrictEquality, // ===
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    // Literal Expressions
    FloatLiteralExp {
        name: String,
        value: f64,
    },
    StringLiteralExp {
        value: String,
    },
    BooleanLiteralExp {
        name: String,
        value: bool,
    },

    IdentExp {
        name: String,
        data_type: DataType,
    },

    UnaryExp {
        operator: UnaryOperator,
        argument: Box<Expression>,
    },

    BinaryExp {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

impl Expression {
    pub fn get_data_type(&self) -> DataType {
        match self {
            Expression::FloatLiteralExp { name: _, value: _ } => return DataType::Float,
            Expression::StringLiteralExp { value: _ } => return DataType::String,
            Expression::BooleanLiteralExp { name: _, value: _ } => return DataType::Boolean,

            Expression::IdentExp { name: _, data_type } => return data_type.clone(),

            Expression::UnaryExp {
                operator,
                argument: _,
            } => match operator {
                UnaryOperator::Bang => return DataType::Boolean,
                UnaryOperator::Minus | UnaryOperator::Plus => return DataType::Float,
            },

            Expression::BinaryExp {
                operator,
                left,
                right,
            } => match operator {
                BinaryOperator::Minus
                | BinaryOperator::Star
                | BinaryOperator::Slash
                | BinaryOperator::VerticalBar
                | BinaryOperator::Caret
                | BinaryOperator::Ampersand => return DataType::Float,

                BinaryOperator::Plus => {
                    let left_data_type = left.get_data_type();
                    let right_data_type = right.get_data_type();

                    if left_data_type == DataType::String || right_data_type == DataType::String {
                        return DataType::String;
                    } else {
                        return DataType::Float;
                    }
                }

                BinaryOperator::StrictEquality => return DataType::Boolean,
            },
        }
    }
}

pub fn convert_unary_op_to_token(op: &UnaryOperator) -> Token {
    match op {
        UnaryOperator::Plus => return Token::Plus,
        UnaryOperator::Bang => return Token::Bang,
        UnaryOperator::Minus => return Token::Minus,
    }
}

pub fn convert_token_to_unary_op(op: &Token) -> Result<UnaryOperator, String> {
    match op {
        Token::Plus => return Ok(UnaryOperator::Plus),
        Token::Minus => return Ok(UnaryOperator::Minus),
        Token::Bang => return Ok(UnaryOperator::Bang),

        tok => {
            return Err(format!(
                "Cannot convert token : {:?} to Unary operator",
                tok
            ))
        }
    }
}

pub fn convert_binary_op_to_token(op: &BinaryOperator) -> Token {
    match op {
        BinaryOperator::Plus => return Token::Plus,
        BinaryOperator::Minus => return Token::Minus,
        BinaryOperator::Star => return Token::Star,
        BinaryOperator::Slash => return Token::Slash,

        BinaryOperator::VerticalBar => return Token::VerticalBar,
        BinaryOperator::Caret => return Token::Caret,
        BinaryOperator::Ampersand => return Token::Ampersand,

        BinaryOperator::StrictEquality => return Token::StrictEquality,
    }
}

pub fn convert_token_to_binary_op(op: Token) -> Result<BinaryOperator, String> {
    match op {
        Token::Plus => return Ok(BinaryOperator::Plus),
        Token::Minus => return Ok(BinaryOperator::Minus),
        Token::Star => return Ok(BinaryOperator::Star),
        Token::Slash => return Ok(BinaryOperator::Slash),

        Token::VerticalBar => return Ok(BinaryOperator::VerticalBar),
        Token::Caret => return Ok(BinaryOperator::Caret),
        Token::Ampersand => return Ok(BinaryOperator::Ampersand),

        Token::StrictEquality => return Ok(BinaryOperator::StrictEquality),

        tok => {
            return Err(format!(
                "Cannot convert token : {:?} to Binary operator",
                tok
            ))
        }
    }
}
