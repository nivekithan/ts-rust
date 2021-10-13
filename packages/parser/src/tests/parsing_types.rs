use ast::data_type::DataType;
use lexer::convert_to_token;

use crate::parser::Parser;

#[test]
fn test_string_type_declaration() {
    let input = "string";

    let tokens = convert_to_token(input);
    let mut parser = Parser::new(&tokens);
    let data_type = parser.parse_type_declaration(1);

    assert_eq!(data_type, Ok(DataType::String));
}

#[test]
fn test_boolean_type_declaration() {
    let input = "boolean";

    let tokens = convert_to_token(input);
    let mut parser = Parser::new(&tokens);
    let data_type = parser.parse_type_declaration(1);

    assert_eq!(data_type, Ok(DataType::Boolean));
}

#[test]
fn test_number_type_declaration() {
    let input = "number";

    let tokens = convert_to_token(input);
    let mut parser = Parser::new(&tokens);
    let data_type = parser.parse_type_declaration(1);

    assert_eq!(data_type, Ok(DataType::Float));
}

#[test]
fn test_grouped_type_declaration() {
    let input = "(string)";

    let tokens = convert_to_token(input);
    let mut parser = Parser::new(&tokens);
    let data_type = parser.parse_type_declaration(1);

    assert_eq!(data_type, Ok(DataType::String));
}

#[test]
fn test_array_type() {
    let input = "string[][]";

    let tokens = convert_to_token(input);
    let mut parser = Parser::new(&tokens);
    let data_type = parser.parse_type_declaration(1);

    match &data_type {
        Err(s) => println!("{}", s),
        _ => {}
    }
    assert_eq!(
        data_type,
        Ok(DataType::ArrayType {
            base_type: Box::new(DataType::ArrayType {
                base_type: Box::new(DataType::String)
            })
        })
    );
}
