pub mod data_type;
pub mod declaration;
pub mod expression;

use data_type::DataType;
use declaration::{Declaration, VariableAssignmentOperator, VariableDeclarationKind};
use expression::{BinaryOperator, Expression, UnaryOperator};

#[derive(Debug, PartialEq, Clone)]
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
            value: name.to_string(),
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
    pub fn new_variable_declaration(
        ident_name: &str,
        exp: Expression,
        kind: VariableDeclarationKind,
    ) -> Ast {
        return Ast::Declaration(Declaration::VariableDeclaration {
            ident_name: ident_name.to_string(),
            exp,
            kind,
        });
    }

    pub fn new_variable_assignment(
        ident_name: &str,
        operator: VariableAssignmentOperator,
        exp: Expression,
    ) -> Ast {
        return Ast::Declaration(Declaration::VariableAssignment {
            operator,
            ident_name: ident_name.to_string(),
            exp,
        });
    }

    pub fn new_if_block(condition: Expression, blocks: Vec<Ast>) -> Ast {
        if let DataType::Boolean = condition.get_data_type() {
            return Ast::Declaration(Declaration::IfBlockDeclaration {
                condition,
                block: Box::new(blocks),
            });
        } else {
            panic!("Condition can only be expression whose datatype is boolean")
        }
    }
}

impl Ast {
    pub fn get_data_type(&self) -> Result<DataType, String> {
        match self {
            Ast::Expression(exp) => {
                return Ok(exp.get_data_type());
            }
            Ast::Declaration(dec) => {
                return Err(format!(
                    "There is no datatype associated with declaration {:?}",
                    dec
                ));
            }
        }
    }
}
