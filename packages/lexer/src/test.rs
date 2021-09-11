use crate::convert_to_token;
use crate::token;
use token::KeywordKind;
use token::LiteralKind;
use token::Token;

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
        AngleOpenBracket,
        AngleCloseBracket,
        CurveOpenBracket,
        CurveCloseBracket,
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
    let 

    true
    false
    
    if";

    let expected_output: Vec<Token> = vec![
        Keyword(KeywordKind::Const),
        Keyword(KeywordKind::Let),
        Keyword(KeywordKind::True),
        Keyword(KeywordKind::False),
        Keyword(KeywordKind::If),
        Eof,
    ];

    let actual_output = convert_to_token(input);

    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_literals() {
    use Token::*;

    let input = "
    1234
    '12'
    \"12\"
    `12`
    ";

    let expected_output = vec![
        Literal(LiteralKind::Float {
            name: String::from("1234"),
            value: 1234.0,
        }),
        Literal(LiteralKind::String {
            name: String::from("12"),
        }),
        Literal(LiteralKind::String {
            name: String::from("12"),
        }),
        Literal(LiteralKind::String {
            name: String::from("12"),
        }),
        Eof,
    ];

    let actual_output = convert_to_token(input);

    assert_eq!(expected_output, actual_output)
}
