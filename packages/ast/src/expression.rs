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
    StrictNotEqual, // !==

    LessThan,        // <
    LessThanOrEqual, // <=

    GreaterThan,        // >
    GreaterThanOrEqual, // >=
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
    ArrayLiteral {
        expression: Box<Vec<Expression>>,
        expression_data_type: DataType,
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
            Expression::ArrayLiteral {
                expression: _,
                expression_data_type: data_type,
            } => {
                return DataType::ArrayType {
                    base_type: Box::new(data_type.clone()),
                }
            }

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

                BinaryOperator::StrictEquality
                | BinaryOperator::StrictNotEqual
                | BinaryOperator::LessThan
                | BinaryOperator::LessThanOrEqual
                | BinaryOperator::GreaterThan
                | BinaryOperator::GreaterThanOrEqual => return DataType::Boolean,
            },
        }
    }
}
