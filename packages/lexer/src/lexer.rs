use std::str::Chars;

use super::token::Token;

pub(crate) struct Lexer<'a> {
    content: Chars<'a>,
    cur_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Lexer<'a> {
        let content = input.chars();

        return Lexer {
            content,
            cur_char: None,
        };
    }

    pub(crate) fn next_token(&mut self) -> Token {
        self.eat_whitespace();

        let cur_char = self.cur_char;

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
                    if is_letter(&char) {
                        let ident_name = self.read_identifier();
                        return Ident { name: ident_name };
                    }

                    return Illegal;
                }
            }
        }
    }

    fn next(&mut self) -> Option<char> {
        let next_cur = self.content.next();
        self.cur_char = next_cur;

        return next_cur;
    }

    fn eat_whitespace(&mut self) {
        let cur_char = self.next();

        if let Some(mut c) = cur_char {
            while is_whitespace(&c) {
                let next_char = self.next();
                if let Some(ch) = next_char {
                    c = ch
                } else {
                    break;
                }
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut ident_name = self.cur_char.expect("unreachable").to_string();
        let next_cur = self.next();

        match next_cur {
            None => return ident_name,
            Some(mut c) => {
                while is_letter(&c) {
                    ident_name.push(c);

                    let next_cur = self.next();

                    match next_cur {
                        None => return ident_name,
                        Some(ch) => c = ch,
                    }
                }

                return ident_name;
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

fn is_letter(c: &char) -> bool {
    return matches!(
        c,
         'a'..='z'
        | 'A'..='Z'
        | '_'
    );
}
