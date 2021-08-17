#[derive(Debug, PartialEq, Clone, Copy)]
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
}
