use crate::{data_type::DataType, expression::Expression};

pub struct ConstVariableDeclaration {
    pub ident: String,
    pub expression: Expression,
}

impl ConstVariableDeclaration {
    pub fn get_data_type(&self) -> DataType {
        return self.expression.get_data_type();
    }
}

#[cfg(test)]
mod const_variable_declaration {
    use crate::{data_type::DataType, expression::{literal_expression::LiteralExpression, Expression}};

    use super::ConstVariableDeclaration;

    #[test]
    fn test_const_variable() {
        let const_variable_declaration = ConstVariableDeclaration {
            ident: "name".to_string(),
            expression: Expression::Literal(LiteralExpression::String {
                name: "12".to_string(),
            }),
        };

        let expected_data_type = DataType::String;
    
        let actual_data_type = const_variable_declaration.get_data_type();

        assert_eq!(expected_data_type, actual_data_type);
    }

}
