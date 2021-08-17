#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Semi, // ;

    Illegal, // Unknown token
    Eof,     // End of File
}
