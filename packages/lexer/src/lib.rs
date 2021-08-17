mod lexer;
mod token;

use crate::lexer::Lexer;
use crate::token::Token;

#[cfg(test)]
mod test;

pub fn convert_to_token(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    let mut tokens: Vec<Token> = Vec::new();

    loop {
        let token = lexer.next_token();

        tokens.push(token);

        if let Token::Eof = token {
            break;
        }
    }

    return tokens;
}
