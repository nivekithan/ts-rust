use crate::token::KeywordKind;

use super::convert_to_token;
use super::Token;

#[test]
fn test_operators() {
    use Token::*;

    let input = "
    =
    
    ;
    :

    {
    }

    (
    )

    ,

    !

    +
    -
    *
    /

    |
    ^
    &
    ";

    let expected_output: Vec<Token> = vec![
        Assign,
        SemiColon,
        Colon,
        LeftBrace,
        RightBrace,
        LeftBracket,
        RightBracket,
        Comma,
        Bang,
        Plus,
        Minus,
        Star,
        Slash,
        VerticalBar,
        Caret,
        Ampersand,
        Eof,
    ];

    let actual_output = convert_to_token(input);

    assert_eq!(expected_output.len(), actual_output.len());
    assert_eq!(expected_output, actual_output)
}

#[test]
fn test_ident() {
    use Token::*;

    let input = "
    whats_my_name
    hoo
    hay";

    let expected_output: Vec<Token> = vec![
        Ident {
            name: String::from("whats_my_name"),
        },
        Ident {
            name: String::from("hoo"),
        },
        Ident {
            name: String::from("hay"),
        },
        Eof,
    ];

    let actual_output = convert_to_token(input);

    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_keyword() {
    use Token::*;

    let input = "
    const
    true
    false";

    let expected_output: Vec<Token> = vec![
        Keyword {
            kind: KeywordKind::Const,
        },
        Keyword {
            kind: KeywordKind::True,
        },
        Keyword {
            kind: KeywordKind::False,
        },
        Eof,
    ];

    let actual_output = convert_to_token(input);

    assert_eq!(expected_output, actual_output);
}
