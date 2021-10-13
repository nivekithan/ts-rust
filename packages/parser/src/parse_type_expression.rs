use ast::data_type::DataType;
use lexer::token::Token;

use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_type_declaration(&mut self, precedence: usize) -> Result<DataType, String> {
        let mut prefix_data_type = self.get_prefix_type()?;

        let next_token = self.get_cur_token()?.clone();

        while precedence < Parser::get_type_non_prefix_precedence(&next_token) {
            let non_prefix_data_type = self.get_non_prefix_type(prefix_data_type)?;

            match non_prefix_data_type {
                Ok(data) => prefix_data_type = data,
                Err(data) => {
                    prefix_data_type = data;
                    break;
                }
            }
        }

        return Ok(prefix_data_type);
    }

    pub(crate) fn get_prefix_type(&mut self) -> Result<DataType, String> {
        let cur_token = self.get_cur_token()?;

        match cur_token {
            Token::Ident { name } => {
                let data_type = {
                    if name == "string" {
                        DataType::String
                    } else if name == "boolean" {
                        DataType::Boolean
                    } else if name == "number" {
                        DataType::Float
                    } else {
                        return Err(format!("Unknown ident name : {:?}", name));
                    }
                };

                self.next(); // consumes ident
                return Ok(data_type);
            }

            Token::CurveOpenBracket => {
                self.next(); // consumes (

                let grouped_data_type = self.parse_type_declaration(1)?;

                self.assert_cur_token(&Token::CurveCloseBracket)?;

                self.next(); // consumes )
                return Ok(grouped_data_type);
            }

            _ => {
                return Err(format!(
                    "Token {:?} does not have associated type prefix function",
                    cur_token
                ))
            }
        }
    }

    pub(crate) fn get_non_prefix_type(
        &mut self,
        left: DataType,
    ) -> Result<Result<DataType, DataType>, String> {
        let cur_tok = self.get_cur_token()?;

        match cur_tok {
            _ => return Ok(Err(left)),
        }
    }

    pub(crate) fn get_type_prefix_precedence(token: &Token) -> usize {
        match token {
            _ => 1,
        }
    }

    pub(crate) fn get_type_non_prefix_precedence(token: &Token) -> usize {
        match token {
            _ => 1,
        }
    }
}
