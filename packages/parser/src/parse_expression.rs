use ast::{data_type::DataType, expression::Expression};
use lexer::token::{KeywordKind, LiteralKind, Token};

use crate::{
    parser::Parser,
    symbol_table::SymbolContext,
    utils::{convert_token_to_binary_operator, convert_token_to_unary_operator},
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(
        &mut self,
        precedence: usize,
        context: &SymbolContext,
    ) -> Result<Expression, String> {
        let mut prefix_fun = self.get_prefix_exp(context)?;
        let next_token = self.get_cur_token()?.clone();

        while next_token != Token::SemiColon
            && precedence < Parser::get_non_prefix_precedence(&next_token)
        {
            let infix_fun = self.get_non_prefix_exp(prefix_fun, context)?;

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

        return Ok(prefix_fun);
    }

    pub(crate) fn get_prefix_exp(&mut self, context: &SymbolContext) -> Result<Expression, String> {
        let cur_token = self.get_cur_token()?;

        match cur_token {
            Token::Plus | Token::Minus | Token::Bang => {
                return self.parse_generic_unary_expression(context);
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

            Token::Ident { name } => {
                // let name = format!("{}{}", name, context.suffix);

                if let Some(sym_meta) = context.get(&name) {
                    let suffix_name = format!("{}{}", name, context.suffix);

                    let exp = Ok(Expression::IdentExp {
                        name: suffix_name,
                        data_type: sym_meta.data_type.clone(),
                    });

                    self.next(); // Consumes ident

                    return exp;
                } else {
                    return Err(format!("There is no variable defined with name {}", name));
                }
            }

            Token::CurveOpenBracket => {
                self.next(); // consume (

                let grouped_exp = self.parse_expression(1, context)?;

                let cur_tok = self.get_cur_token().unwrap();

                if cur_tok == &Token::Eof {
                    println!("Contents : {:?}", self.content);
                }

                assert_eq!(cur_tok, &Token::CurveCloseBracket);
                self.next(); // consumes )

                return Ok(grouped_exp);
            }

            Token::BoxOpenBracket => {
                self.next(); // consumes [

                let mut expressions: Vec<Expression> = vec![];

                let mut there_is_comma = true;

                while self.get_cur_token()? != &Token::AngleCloseBracket && there_is_comma {
                    let item = self.parse_expression(1, context)?;

                    let tok = self.get_cur_token()?;

                    if tok == &Token::Comma {
                        there_is_comma = true;
                        self.next();
                    } else {
                        there_is_comma = false;
                    }
                    expressions.push(item);
                }

                if there_is_comma {
                    self.next(); // consumes ]
                } else {
                    self.assert_cur_token(&Token::BoxCloseBracket)?;
                    self.next(); // consumes ]
                }

                if expressions.len() <= 0 {
                    return Err(format!("Creating array with 0 items is not yet supported"));
                }

                let mut data_type = DataType::Unknown;

                let matched = expressions.iter().enumerate().all(|(i, exp)| {
                    if i == 0 {
                        data_type = exp.get_data_type();
                        return true;
                    }

                    return data_type == exp.get_data_type();
                });

                if !matched {
                    return Err(format!(
                        "Expected all expressions to have same datatype in array"
                    ));
                }

                return Ok(Expression::ArrayLiteral {
                    expression: Box::new(expressions),
                    expression_data_type: data_type,
                });
            }

            tok => {
                return Err(format!(
                    "Given token {:?} does not have not a prefix function",
                    tok
                ))
            }
        }
    }

    pub(crate) fn parse_generic_unary_expression(
        &mut self,
        context: &SymbolContext,
    ) -> Result<Expression, String> {
        let cur_token = self.get_cur_token()?.clone();
        let precedence = Parser::get_prefix_precedence(&cur_token);

        self.next(); // consumes cur_token

        let arg_exp = self.parse_expression(precedence, context)?;
        return Ok(Expression::UnaryExp {
            operator: convert_token_to_unary_operator(&cur_token),
            argument: Box::new(arg_exp),
        });
    }

    pub(crate) fn get_non_prefix_exp(
        &mut self,
        left: Expression,
        context: &SymbolContext,
    ) -> Result<Result<Expression, Expression>, String> {
        let non_prefix_token = self.get_cur_token().unwrap();

        match non_prefix_token {
            Token::Plus
            | Token::Minus
            | Token::Star
            | Token::Slash
            | Token::VerticalBar
            | Token::Caret
            | Token::Ampersand
            | Token::StrictEquality
            | Token::StrictNotEqual
            | Token::LessThan
            | Token::LessThanOrEqual
            | Token::GreaterThan
            | Token::GreaterThanOrEqual => {
                let exp = self.parse_generic_binary_expression(left, context)?;
                return Ok(Ok(exp));
            }

            _ => return Ok(Err(left)),
        }
    }

    pub(crate) fn parse_generic_binary_expression(
        &mut self,
        left: Expression,
        context: &SymbolContext,
    ) -> Result<Expression, String> {
        let cur_tok = self.get_cur_token()?.clone();

        let precedence = Parser::get_non_prefix_precedence(&cur_tok);

        self.next(); // consumes cur_tok which is binary_tok

        let right_exp = Box::new(self.parse_expression(precedence, context)?);
        return Ok(Expression::BinaryExp {
            operator: convert_token_to_binary_operator(&cur_tok),
            left: Box::new(left),
            right: right_exp,
        });
    }

    pub(crate) fn get_prefix_precedence(token: &Token) -> usize {
        match token {
            Token::Plus | Token::Minus | Token::Bang => return 17,

            _ => return 1,
        }
    }

    pub(crate) fn get_non_prefix_precedence(token: &Token) -> usize {
        match token {
            Token::Star | Token::Slash => return 15,

            Token::Plus | Token::Minus => return 14,

            Token::LessThan
            | Token::LessThanOrEqual
            | Token::GreaterThan
            | Token::GreaterThanOrEqual => return 12,

            Token::StrictEquality | Token::StrictNotEqual => return 11,
            Token::Ampersand => return 10,
            Token::Caret => return 9,
            Token::VerticalBar => return 8,

            _ => return 1,
        }
    }
}
