use crate::token::LiteralKind;

use super::token::KeywordKind;
use core::panic;
use std::str::Chars;

use super::token::Token;

pub(crate) struct Lexer<'a> {
    content: Chars<'a>,
    cur_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Lexer<'a> {
        let content = input.chars();

        let mut lexer = Lexer {
            content,
            cur_char: None,
        };

        lexer.next();

        return lexer;
    }

    pub(crate) fn next_token(&mut self) -> Token {
        self.eat_whitespace();

        let cur_char = self.cur_char;

        match cur_char {
            None => Token::Eof,
            Some(char) => {
                use Token::*;

                if char == ';' {
                    self.next(); // consumes ;
                    return SemiColon;
                } else if char == ':' {
                    self.next(); // consumes :
                    return Colon;
                } else if char == '=' {
                    self.next(); // consumes =
                    return Assign;
                } else if char == '{' {
                    self.next(); // consumes {
                    return AngleOpenBracket;
                } else if char == '}' {
                    self.next(); // consumes }
                    return AngleCloseBracket;
                } else if char == '(' {
                    self.next(); // consumes (
                    return CurveOpenBracket;
                } else if char == ')' {
                    self.next(); // consumes )
                    return CurveCloseBracket;
                } else if char == ',' {
                    self.next(); // consumes ,
                    return Comma;
                } else if char == '!' {
                    self.next(); // consumes !
                    return Bang;
                } else if char == '+' {
                    self.next(); // consumes +
                    return Plus;
                } else if char == '-' {
                    self.next(); // consumes -
                    return Minus;
                } else if char == '*' {
                    self.next(); // consumes +
                    return Star;
                } else if char == '/' {
                    self.next(); // consumes /
                    return Slash;
                } else if char == '|' {
                    self.next(); // consumes |
                    return VerticalBar;
                } else if char == '^' {
                    self.next(); // consumes ^
                    return Caret;
                } else if char == '&' {
                    self.next(); // consumes &
                    return Ampersand;
                } else if char == '\'' {
                    let string_name = self.read_string('\'');
                    return Literal(LiteralKind::String { name: string_name });
                } else if char == '\"' {
                    let string_name = self.read_string('\"');
                    return Literal(LiteralKind::String { name: string_name });
                } else if char == '`' {
                    let string_name = self.read_string('`');
                    return Literal(LiteralKind::String { name: string_name });
                } else {
                    if is_letter(&char) {
                        let ident_name = self.read_identifier();
                        let may_be_keyword = is_keyword(&ident_name);

                        match may_be_keyword {
                            IsKeyword::Yes(kind) => return Keyword(kind),
                            IsKeyword::No => return Ident { name: ident_name },
                        }
                    } else if is_digit(&char) {
                        let digit_name = self.read_digit();
                        let digit_value: f64 = digit_name.parse().unwrap();

                        return Literal(LiteralKind::Float {
                            name: digit_name,
                            value: digit_value,
                        });
                    }
                    self.next(); // consumes Illegal
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

    // If cur_char is whitespace it wil eat char until cur_char is not whitespace
    fn eat_whitespace(&mut self) {
        let cur_char = self.cur_char;

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

    // After executing this function the cur_char wont be a letter
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
    // After executing this function the cur_char wont be a digit
    fn read_digit(&mut self) -> String {
        let mut digit_name = self.cur_char.expect("unreachable").to_string();

        let next_cur = self.next();

        match next_cur {
            None => return digit_name,
            Some(mut c) => {
                while is_digit(&c) {
                    digit_name.push(c);

                    let next_cur = self.next();

                    match next_cur {
                        None => return digit_name,
                        Some(ch) => c = ch,
                    }
                }

                return digit_name;
            }
        }
    }
    // Assumes the cur_char is starting char of string_literal
    // Ex: ' " `
    // It will end after consuming the end char that
    // cur_char wont be end_char
    fn read_string(&mut self, end_char: char) -> String {
        let mut string_name = String::new();
        loop {
            let next_char = self.next();

            match next_char {
                None => panic!("Lexer error expected {} before end of file", end_char),
                Some(ch) => {
                    if ch == end_char {
                        self.next(); // consumes end_char
                        break;
                    } else {
                        string_name.push(ch)
                    }
                }
            }
        }

        return string_name;
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

fn is_digit(c: &char) -> bool {
    return matches!(c, '0'..='9');
}

pub enum IsKeyword {
    Yes(KeywordKind),
    No,
}

fn is_keyword(word: &String) -> IsKeyword {
    if word == "const" {
        return IsKeyword::Yes(KeywordKind::Const);
    } else if word == "true" {
        return IsKeyword::Yes(KeywordKind::True);
    } else if word == "false" {
        return IsKeyword::Yes(KeywordKind::False);
    } else if word == "let" {
        return IsKeyword::Yes(KeywordKind::Let);
    } else if word == "if" {
        return IsKeyword::Yes(KeywordKind::If);
    } else {
        return IsKeyword::No;
    }
}
