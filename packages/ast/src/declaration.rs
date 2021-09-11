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

    ReVariableAssignment {
        ident_name: String,
        exp: Expression,
    },

    IfBlockDeclaration {
        condition: Expression,
        block: Box<Vec<Ast>>,
    },
}
