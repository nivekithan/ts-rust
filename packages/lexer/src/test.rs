use super::convert_to_token;
use super::Token;

#[test]
fn test_operators() {
    let input = ";";

    let expected_output: Vec<Token> = vec![Token::Semi, Token::Eof];

    let actual_output = convert_to_token(input);

    assert_eq!(expected_output.len(), actual_output.len());
    assert_eq!(expected_output, actual_output)
}
