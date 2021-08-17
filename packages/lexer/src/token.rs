#[derive(Debug, PartialEq)]
pub enum Token {
    Assign, // =

    SemiColon, // ;
    Colon,     // :

    LeftBrace,  // {
    RightBrace, // }

    LeftBracket,  // (
    RightBracket, // )

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

#[derive(Debug, PartialEq)]
pub enum KeywordKind {
    Const,
    True,
    False,
}

#[derive(Debug, PartialEq)]

pub enum LiteralKind {
    Float { name: String, value: f64 },

    String { name: String },
}
