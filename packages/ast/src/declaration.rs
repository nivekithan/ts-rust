use crate::{expression::Expression, Ast};

#[derive(Debug, PartialEq, Clone)]
pub enum VariableDeclarationKind {
    Const,
    Let,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    VariableDeclaration {
        ident_name: String,
        exp: Expression,
        kind: VariableDeclarationKind,
    },

    VariableAssignment {
        ident_name: String,
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
