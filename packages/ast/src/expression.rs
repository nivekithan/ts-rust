use std::collections::HashMap;

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
        expression_data_type: DataType, // Should correspond to DataType::ArrayType
    },
    ObjectLiteral {
        expression: HashMap<String, Expression>,
        data_type: DataType, // should correspond to DataType::ObjectType
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

    ArrayMemberAccess {
        array: Box<Expression>,
        argument: Box<Expression>,
    },

    DotMemberAccess {
        container: Box<Expression>,
        argument: String,
    },
    // Function {
    //     return_type: DataType,
    //     arguments: Vec<DataType>,
    //     block: Box<Vec<Ast>>,
    // },
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
            Expression::ObjectLiteral {
                expression: _,
                data_type,
            } => {
                return data_type.clone();
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

            /*
             * It only validates that datatype of field "array" is of DataType::ArrayType
             * it does not validate weather argument is valid or not
             *
             * */
            Expression::ArrayMemberAccess { argument: _, array } => {
                let data_type = array.get_data_type();

                if let DataType::ArrayType { base_type } = data_type {
                    return base_type.as_ref().clone();
                } else {
                    unreachable!();
                }
            }

            Expression::DotMemberAccess {
                argument,
                container,
            } => {
                let exp_data_type = container.get_data_type();

                match &exp_data_type {
                    DataType::ObjectType{entries} => {
                        let member_data_type = entries.get(argument).expect(format!("There is no member with name {} on Datatype {:?}", argument, exp_data_type).as_str());
                        return member_data_type.clone();
                    },

                    _ => panic!("As of now only expression with datatype Datatype::ObjectType is supported for got expression with datatype {:?}", exp_data_type)
                }
            } // Expression::Function {
              //     arguments,
              //     return_type,
              //     block: _,
              // } => {
              //     return DataType::FunctionType {
              //         arguments: arguments.clone(),
              //         return_type: Box::new(return_type.clone()),
              //     };
              // }
        }
    }
}
