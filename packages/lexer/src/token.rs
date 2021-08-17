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

    Keyword { kind: KeywordKind },
}

#[derive(Debug, PartialEq)]
pub enum KeywordKind {
    Const,
}
