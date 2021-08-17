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
        let cur_char = self.eat_whitespace();

        match cur_char {
            None => Token::Eof,
            Some(char) => {
                use Token::*;

                if char == ';' {
                    return SemiColon;
                } else if char == ':' {
                    return Colon;
                } else if char == '=' {
                    return Assign;
                } else if char == '{' {
                    return LeftBrace;
                } else if char == '}' {
                    return RightBrace;
                } else if char == '(' {
                    return LeftBracket;
                } else if char == ')' {
                    return RightBracket;
                } else if char == ',' {
                    return Comma;
                } else if char == '!' {
                    return Bang;
                } else if char == '+' {
                    return Plus;
                } else if char == '-' {
                    return Minus;
                } else if char == '*' {
                    return Star;
                } else if char == '/' {
                    return Slash;
                } else if char == '|' {
                    return VerticalBar;
                } else if char == '^' {
                    return Caret;
                } else if char == '&' {
                    return Ampersand;
                } else {
                    return Illegal;
                }
            }
        }
    }

    fn eat_whitespace(&mut self) -> Option<char> {
        let cur_char = self.content.next();

        match cur_char {
            None => return None,
            Some(mut c) => {
                while is_whitespace(&c) {
                    let next_char = self.content.next();

                    match next_char {
                        None => return None,
                        Some(ch) => c = ch,
                    }
                }

                return Some(c);
            }
        }
    }
}

// From rust source code
fn is_whitespace(c: &char) -> bool {
    return matches!(
        c,
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR)
    );
}
