use crate::{data_type::DataType, expression::Expression};

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
}

impl Declaration {
    pub fn get_data_type_of_exp(&self) -> DataType {
        match self {
            Declaration::VariableDeclaration {
                ident_name: _,
                exp,
                kind: _,
            } => {
                return exp.get_data_type();
            }

            Declaration::ReVariableAssignment { exp, ident_name: _ } => return exp.get_data_type(),
        }
    }
}
