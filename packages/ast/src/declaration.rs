use crate::{data_type::DataType, expression::Expression};

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    ConstVariableDeclaration { ident_name: String, exp: Expression },
}

impl Declaration {
    pub fn get_data_type_of_exp(&self) -> DataType {
        match self {
            Declaration::ConstVariableDeclaration { ident_name: _, exp } => {
                return exp.get_data_type();
            }
        }
    }
}
