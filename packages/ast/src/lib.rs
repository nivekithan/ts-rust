pub mod data_type;
pub mod declaration;
pub mod expression;

use data_type::DataType;
use declaration::{
    BlockWithCondition, Declaration, VariableAssignmentOperator, VariableDeclarationKind,
};
use expression::{BinaryOperator, Expression, UnaryOperator};
use indexmap::IndexMap;

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

    pub fn new_import_declaration(ident: IndexMap<String, DataType>, from: &str) -> Ast {
        return Ast::Declaration(Declaration::ImportDeclaration {
            ident,
            from: from.to_string(),
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

    pub fn new_array_member_assignment(
        ident_name: &str,
        member_access_exp: Expression,
        operator: VariableAssignmentOperator,
        exp: Expression,
    ) -> Ast {
        return Ast::Declaration(Declaration::ArrayMemberAssignment {
            operator,
            exp,
            ident_name: ident_name.to_string(),
            member_access_exp,
        });
    }

    pub fn new_if_block(
        if_block: BlockWithCondition,
        else_if_block: Vec<BlockWithCondition>,
        else_block: Option<Box<Vec<Ast>>>,
    ) -> Ast {
        return Ast::Declaration(Declaration::NewIfBlockDeclaration {
            if_block,
            else_block,
            else_if_block,
        });
    }

    pub fn new_while_loop(while_loop: BlockWithCondition) -> Ast {
        return Ast::Declaration(Declaration::WhileLoopDeclaration {
            condition: while_loop.condition,
            block: while_loop.block,
        });
    }

    pub fn new_do_while_loop(do_while_loop: BlockWithCondition) -> Ast {
        return Ast::Declaration(Declaration::DoWhileLoopDeclaration {
            condition: do_while_loop.condition,
            block: do_while_loop.block,
        });
    }

    pub fn new_function_declaration(
        arguments: IndexMap<String, DataType>,
        blocks: Box<Vec<Ast>>,
        ident_name: String,
        return_type: DataType,
    ) -> Ast {
        return Ast::Declaration(Declaration::FunctionDeclaration {
            arguments,
            blocks,
            ident_name,
            return_type,
        });
    }

    pub fn new_return_statement(exp: Option<Expression>) -> Ast {
        return Ast::Declaration(Declaration::ReturnStatement { return_exp: exp });
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
