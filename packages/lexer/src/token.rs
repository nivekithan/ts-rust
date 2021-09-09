#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Assign, // =

    SemiColon, // ;
    Colon,     // :

    AngleOpenBracket,  // {
    AngleCloseBracket, // }

    CurveOpenBracket,  // (
    CurveCloseBracket, // )

    Comma, // ,

    Bang, // !

    Plus,  // +
    Minus, // -
    Star,  // *
    Slash, // /

    VerticalBar, // |
    Caret,       // ^
    Ampersand,   // &

    Illegal, // Unknown token
    Eof,     // End of File

    Ident { name: String },

    Keyword(KeywordKind),

    Literal(LiteralKind),
}

#[derive(Debug, PartialEq, Clone)]
pub enum KeywordKind {
    Const,
    True,
    False,
    Let,
}

#[derive(Debug, PartialEq, Clone)]

pub enum LiteralKind {
    Float { name: String, value: f64 },

    String { name: String },
}

impl Token {
    pub fn get_ident_name(&self) -> Result<&String, String> {
        match self {
            Self::Ident { name } => return Ok(name),
            tok => Err(format!(
                "Expected current token to be ident but got {:?}",
                tok
            )),
        }
    }
}

#[cfg(test)]
mod token_test {
    use super::Token;

    #[test]
    fn test_get_ident_name() {
        let ident_token = Token::Ident {
            name: "name".to_string(),
        };

        let name = &"name".to_string();

        let expected_output: Result<&String, String> = Ok(name);
        let actual_output = ident_token.get_ident_name();

        assert_eq!(expected_output, actual_output);

        let another_token = Token::Ampersand;

        let error = String::from("Expected current token to be ident but got Ampersand");

        let expected_output: Result<&String, String> = Err(error);
        let actual_output = another_token.get_ident_name();

        assert_eq!(expected_output, actual_output);
    }
}
