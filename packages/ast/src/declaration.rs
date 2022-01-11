use indexmap::IndexMap;
use lexer::token::{KeywordKind, Token};

use crate::{data_type::DataType, expression::Expression, Ast};

#[derive(Debug, PartialEq, Clone)]
pub enum VariableDeclarationKind {
    Const,
    Let,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    ImportDeclaration {
        ident: IndexMap<String, DataType>,
        from: String,
    },

    VariableDeclaration {
        ident_name: String,
        exp: Expression,
        kind: VariableDeclarationKind,
    },

    FunctionDeclaration {
        ident_name: String,
        return_type: DataType,
        arguments: IndexMap<String, DataType>,
        blocks: Box<Vec<Ast>>,
    },

    VariableAssignment {
        ident_name: String,
        operator: VariableAssignmentOperator,
        exp: Expression,
    },

    ArrayMemberAssignment {
        ident_name: String,
        member_access_exp: Expression,
        operator: VariableAssignmentOperator,
        exp: Expression,
    },

    NewIfBlockDeclaration {
        if_block: BlockWithCondition,
        else_if_block: Vec<BlockWithCondition>,
        else_block: Option<Box<Vec<Ast>>>,
    },

    WhileLoopDeclaration {
        condition: Expression,
        block: Box<Vec<Ast>>,
    },

    DoWhileLoopDeclaration {
        condition: Expression,
        block: Box<Vec<Ast>>,
    },

    LoopControlFlow {
        keyword: KeywordKind,
    },

    ReturnStatement {
        return_exp: Option<Expression>, // if Option is Option::None then the fn is returning void
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum VariableAssignmentOperator {
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockWithCondition {
    pub condition: Expression,
    pub block: Box<Vec<Ast>>,
}

impl BlockWithCondition {
    pub fn new(condition: Expression, block: Vec<Ast>) -> Self {
        return BlockWithCondition {
            condition,
            block: Box::new(block),
        };
    }
}

impl VariableAssignmentOperator {
    pub fn is_lexer_assignment_operator(token: &Token) -> bool {
        match token {
            Token::Assign
            | Token::MinusAssign
            | Token::PlusAssign
            | Token::SlashAssign
            | Token::StarAssign => true,

            _ => false,
        }
    }
}
