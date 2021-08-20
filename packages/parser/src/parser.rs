use ast::{
    expression::{BinaryOperator, Expression, UnaryOperator},
    Ast,
};
use lexer::token::{KeywordKind, LiteralKind, Token};

pub struct Parser<'a> {
    content: &'a Vec<Token>,
    cur_pos: Option<usize>,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a Vec<Token>) -> Parser<'a> {
        let mut parser = Parser {
            content,
            cur_pos: None,
        };

        parser.next();

        return parser;
    }

    pub fn next_ast(&mut self) -> Ast {
        let first_token = self.get_cur_token().unwrap();

        match first_token {
            Token::Keyword(keyword_kind) => match keyword_kind {
                KeywordKind::Const => {
                    let name = self.next().get_ident_name().unwrap().clone(); // consumes Const

                    self.assert_next_token(&Token::Assign); // consumes ident

                    self.next(); // consumes =

                    let expression = self.parse_expression(1);

                    self.skip_semicolon();
                    return Ast::new_const_variable_declaration(&name, expression);
                }

                _ => panic!(
                    "Update function next_ast\n Unexpected keyword, {:?}",
                    keyword_kind
                ),
            },

            tok => panic!("Update function next_ast\n unknown token, {:?}", tok),
        }
    }

    fn next(&mut self) -> &Token {
        match self.cur_pos {
            None => {
                self.cur_pos = Some(0);
                return &self.content[0];
            }

            Some(value) => {
                if value >= self.content.len() - 1 {
                    panic!("cur_pos is at maximum value")
                }

                let next_value = value + 1;
                self.cur_pos = Some(next_value);
                return &self.content[next_value];
            }
        }
    }

    fn assert_next_token(&mut self, token_type: &Token) {
        let next_token = self.next();

        if next_token != token_type {
            panic!(
                "Expected token type {0:?} but got {1:?}",
                token_type, next_token
            )
        }
    }

    fn skip_semicolon(&mut self) {
        let cur_token = self.get_cur_token().unwrap();

        if let &Token::SemiColon = cur_token {
            self.next();
        }
    }

    pub fn get_cur_token(&self) -> Result<&Token, String> {
        if let Some(size) = self.cur_pos {
            return Ok(&self.content[size]);
        } else {
            return Err(
                "Next method should be called atleast one time before calling get_cur_token "
                    .to_string(),
            );
        }
    }

    // Assumes that only by calling self.next it will get
    // related token for parsing expression

    fn parse_expression(&mut self, precedence: usize) -> Expression {
        let mut prefix_fun = self.get_prefix_exp().unwrap();
        let next_token = self.get_cur_token().unwrap().clone();

        while next_token != Token::SemiColon
            && precedence < Parser::get_non_prefix_precedence(&next_token)
        {
            let infix_fun = self.get_non_prefix_exp(prefix_fun);

            match infix_fun {
                Ok(exp) => {
                    prefix_fun = exp;
                }
                Err(exp) => {
                    prefix_fun = exp;
                    break;
                }
            }
        }

        return prefix_fun;
    }

    fn get_prefix_exp(&mut self) -> Result<Expression, String> {
        let cur_token = self.get_cur_token().unwrap();

        match cur_token {
            Token::Plus => {
                let precedence = Parser::get_prefix_precedence(&Token::Plus);

                self.next(); // consumes +
                let arg_exp = self.parse_expression(precedence);
                return Ok(Expression::UnaryExp {
                    operator: UnaryOperator::Plus,
                    argument: Box::new(arg_exp),
                });
            }

            Token::Minus => {
                let precedence = Parser::get_prefix_precedence(&Token::Minus);

                self.next(); // consumes -
                let arg_exp = self.parse_expression(precedence);
                return Ok(Expression::UnaryExp {
                    operator: UnaryOperator::Minus,
                    argument: Box::new(arg_exp),
                });
            }

            Token::Bang => {
                let precedence = Parser::get_prefix_precedence(&Token::Bang);

                self.next(); //  consumes !
                let arg_exp = self.parse_expression(precedence);
                return Ok(Expression::UnaryExp {
                    operator: UnaryOperator::Bang,
                    argument: Box::new(arg_exp),
                });
            }

            Token::Literal(literal_kind) => match literal_kind {
                LiteralKind::Float { name, value } => {
                    let name = name.to_string();
                    let value = *value;

                    self.next(); // consumes Float

                    return Ok(Expression::FloatLiteralExp { name, value });
                }

                LiteralKind::String { name } => {
                    let name = name.to_string();

                    self.next(); // consumes string

                    return Ok(Expression::StringLiteralExp { value: name });
                }
            },

            Token::Keyword(keyword_kind) => match keyword_kind {
                KeywordKind::True => {
                    self.next(); // consumes true

                    return Ok(Expression::BooleanLiteralExp {
                        name: "true".to_string(),
                        value: true,
                    });
                }

                KeywordKind::False => {
                    self.next(); // consumes false

                    return Ok(Expression::BooleanLiteralExp {
                        name: "false".to_string(),
                        value: false,
                    });
                }

                _ => {
                    return Err(format!(
                        "Given keyword does not have a prefix function {:?}",
                        keyword_kind
                    ))
                }
            },

            Token::CurveOpenBracket => {
                self.next(); // consume (

                let grouped_exp = self.parse_expression(1);

                let cur_tok = self.get_cur_token().unwrap();

                if cur_tok == &Token::Eof {
                    println!("Contents : {:?}", self.content);
                }

                assert_eq!(cur_tok, &Token::CurveCloseBracket);
                self.next(); // consumes )

                return Ok(grouped_exp);
            }

            tok => {
                return Err(format!(
                    "Given token {:?} does not have not a prefix function",
                    tok
                ))
            }
        }
    }

    fn get_non_prefix_exp(&mut self, left: Expression) -> Result<Expression, Expression> {
        let non_prefix_token = self.get_cur_token().unwrap();

        match non_prefix_token {
            Token::Plus => {
                let precedence = Parser::get_non_prefix_precedence(&Token::Plus);

                self.next(); // consumes +

                let right_exp = Box::new(self.parse_expression(precedence));
                return Ok(Expression::BinaryExp {
                    operator: BinaryOperator::Plus,
                    left: Box::new(left),
                    right: right_exp,
                });
            }

            Token::Minus => {
                let precedence = Parser::get_non_prefix_precedence(&Token::Minus);

                self.next(); // consumes -

                let right_exp = Box::new(self.parse_expression(precedence));
                return Ok(Expression::BinaryExp {
                    operator: BinaryOperator::Minus,
                    left: Box::new(left),
                    right: right_exp,
                });
            }

            Token::Star => {
                let precedence = Parser::get_non_prefix_precedence(&Token::Star);

                self.next(); // consumes *

                let right_exp = Box::new(self.parse_expression(precedence));
                return Ok(Expression::BinaryExp {
                    operator: BinaryOperator::Star,
                    left: Box::new(left),
                    right: right_exp,
                });
            }

            Token::Slash => {
                let precedence = Parser::get_non_prefix_precedence(&Token::Slash);

                self.next(); // consumes /

                let right_exp = Box::new(self.parse_expression(precedence));
                return Ok(Expression::BinaryExp {
                    operator: BinaryOperator::Slash,
                    left: Box::new(left),
                    right: right_exp,
                });
            }

            Token::VerticalBar => {
                let precedence = Parser::get_non_prefix_precedence(&Token::VerticalBar);

                self.next(); // consumes |

                let right_exp = Box::new(self.parse_expression(precedence));
                return Ok(Expression::BinaryExp {
                    operator: BinaryOperator::VerticalBar,
                    left: Box::new(left),
                    right: right_exp,
                });
            }

            Token::Caret => {
                let precedence = Parser::get_non_prefix_precedence(&Token::Caret);

                self.next(); // consumes ^

                let right_exp = Box::new(self.parse_expression(precedence));
                return Ok(Expression::BinaryExp {
                    operator: BinaryOperator::Caret,
                    left: Box::new(left),
                    right: right_exp,
                });
            }

            Token::Ampersand => {
                let precedence = Parser::get_non_prefix_precedence(&Token::Ampersand);

                self.next(); // consumes &

                let right_exp = Box::new(self.parse_expression(precedence));
                return Ok(Expression::BinaryExp {
                    operator: BinaryOperator::Ampersand,
                    left: Box::new(left),
                    right: right_exp,
                });
            }

            _ => return Err(left),
        }
    }

    fn get_prefix_precedence(token: &Token) -> usize {
        match token {
            Token::Plus | Token::Minus | Token::Bang => return 17,

            _ => return 1,
        }
    }

    fn get_non_prefix_precedence(token: &Token) -> usize {
        match token {
            Token::Star | Token::Slash => return 15,

            Token::Plus | Token::Minus => return 14,

            Token::Ampersand => return 10,
            Token::Caret => return 9,
            Token::VerticalBar => return 8,

            _ => return 1,
        }
    }
}
