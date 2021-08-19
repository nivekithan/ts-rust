pub mod data_type;
pub mod declaration;
pub mod expression;

use data_type::DataType;
use declaration::Declaration;
use expression::{BinaryOperator, Expression, UnaryOperator};

#[derive(Debug, PartialEq)]
pub enum Ast {
    Expression(Expression),
    Declaration(Declaration),
}

// constructors

impl Ast {
    pub fn new_float_literal(name: &String, value: f64) -> Ast {
        return Ast::Expression(Expression::FloatLiteralExp {
            name: name.to_string(),
            value,
        });
    }

    pub fn new_boolean_literal(name: &String, value: bool) -> Ast {
        return Ast::Expression(Expression::BooleanLiteralExp {
            name: name.to_string(),
            value,
        });
    }

    pub fn new_string_literal(name: &String) -> Ast {
        return Ast::Expression(Expression::StringLiteralExp {
            name: name.to_string(),
        });
    }

    pub fn new_unary_exp(argument: Box<Expression>, operator: UnaryOperator) -> Ast {
        return Ast::Expression(Expression::UnaryExp { operator, argument });
    }

    pub fn new_binary_exp(
        left: Box<Expression>,
        right: Box<Expression>,
        operator: BinaryOperator,
    ) -> Ast {
        return Ast::Expression(Expression::BinaryExp {
            operator,
            left,
            right,
        });
    }

    pub fn new_const_variable_declaration(ident_name: &String, exp: Expression) -> Ast {
        return Ast::Declaration(Declaration::ConstVariableDeclaration {
            ident_name: ident_name.to_string(),
            exp,
        });
    }
}

impl Ast {
    pub fn get_data_type(&self) -> DataType {
        match self {
            Ast::Expression(exp) => {
                return exp.get_data_type();
            }
            Ast::Declaration(dec) => {
                return dec.get_data_type_of_exp();
            }
        }
    }
}
