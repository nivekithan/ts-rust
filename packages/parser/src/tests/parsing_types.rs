use ast::data_type::DataType;
use indexmap::IndexMap;
use lexer::convert_to_token;

use crate::{parser::Parser, traits::DummyImportResolver};

#[test]
fn test_string_type_declaration() {
    let input = "string";

    let tokens = convert_to_token(input);
    let mut resolver = DummyImportResolver::new();
    let mut parser = Parser::new(&tokens, &mut resolver, None, 0);
    let data_type = parser.parse_type_declaration(1);

    assert_eq!(data_type, Ok(DataType::String));
}

#[test]
fn test_boolean_type_declaration() {
    let input = "boolean";

    let tokens = convert_to_token(input);
    let mut resolver = DummyImportResolver::new();
    let mut parser = Parser::new(&tokens, &mut resolver, None, 0);
    let data_type = parser.parse_type_declaration(1);

    assert_eq!(data_type, Ok(DataType::Boolean));
}

#[test]
fn test_number_type_declaration() {
    let input = "number";

    let tokens = convert_to_token(input);
    let mut resolver = DummyImportResolver::new();
    let mut parser = Parser::new(&tokens, &mut resolver, None, 0);
    let data_type = parser.parse_type_declaration(1);

    assert_eq!(data_type, Ok(DataType::Float));
}

#[test]
fn test_void_type() {
    let input = "void";

    let tokens = convert_to_token(input);
    let mut resolver = DummyImportResolver::new();
    let mut parser = Parser::new(&tokens, &mut resolver, None, 0);
    let data_type = parser.parse_type_declaration(1);

    assert_eq!(data_type, Ok(DataType::Void));
}

#[test]
fn test_grouped_type_declaration() {
    let input = "(string)";

    let tokens = convert_to_token(input);
    let mut resolver = DummyImportResolver::new();
    let mut parser = Parser::new(&tokens, &mut resolver, None, 0);
    let data_type = parser.parse_type_declaration(1);

    assert_eq!(data_type, Ok(DataType::String));
}

#[test]
fn test_array_type() {
    let input = "string[][]";

    let tokens = convert_to_token(input);
    let mut resolver = DummyImportResolver::new();
    let mut parser = Parser::new(&tokens, &mut resolver, None, 0);
    let data_type = parser.parse_type_declaration(1);

    assert_eq!(
        data_type,
        Ok(DataType::ArrayType {
            base_type: Box::new(DataType::ArrayType {
                base_type: Box::new(DataType::String)
            })
        })
    );
}

#[test]
fn test_object_type() {
    let input = "{a : string, b : number , c : string[], d : boolean}";

    let tokens = convert_to_token(input);
    let mut resolver = DummyImportResolver::new();
    let mut parser = Parser::new(&tokens, &mut resolver, None, 0);
    let data_type = parser.parse_type_declaration(1);

    let mut data_type_entries: IndexMap<String, DataType> = IndexMap::new();

    data_type_entries.insert("a".to_string(), DataType::String);
    data_type_entries.insert("b".to_string(), DataType::Float);
    data_type_entries.insert(
        "c".to_string(),
        DataType::ArrayType {
            base_type: Box::new(DataType::String),
        },
    );
    data_type_entries.insert("d".to_string(), DataType::Boolean);

    assert_eq!(
        data_type,
        Ok(DataType::ObjectType {
            entries: data_type_entries
        })
    );
}

#[test]
fn test_function_type() {
    let input = "(a : string, b : string) => number";

    let tokens = convert_to_token(input);
    let mut resolver = DummyImportResolver::new();
    let mut parser = Parser::new(&tokens, &mut resolver, None, 0);
    let data_type = parser.parse_type_declaration(1);

    let arguments = vec![DataType::String, DataType::String];

    assert_eq!(
        data_type,
        Ok(DataType::FunctionType {
            arguments,
            return_type: Box::new(DataType::Float)
        })
    );
}
