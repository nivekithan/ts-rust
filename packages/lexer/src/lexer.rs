use std::str::Chars;

use super::token::Token;

pub(crate) struct Lexer<'a> {
    content: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Lexer<'a> {
        let content = input.chars();

        return Lexer { content };
    }

    pub(crate) fn next_token(&mut self) -> Token {
        let cur_char = self.content.next();

        return match cur_char {
            None => Token::Eof,
            Some(char) => {
                if char == ';' {
                    return Token::Semi;
                } else {
                    return Token::Eof;
                }
            }
        };
    }
}
