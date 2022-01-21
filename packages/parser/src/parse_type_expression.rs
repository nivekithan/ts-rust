use ast::data_type::DataType;
use indexmap::IndexMap;
use lexer::token::Token;

use crate::{parser::Parser, traits::ImportResolver, utils::convert_index_map_to_vec};

impl<'a, R: ImportResolver> Parser<'a, R> {
    pub(crate) fn parse_type_declaration(&mut self, precedence: usize) -> Result<DataType, String> {
        let mut prefix_data_type = self.get_prefix_type()?;

        let next_token = self.get_cur_token()?.clone();

        while precedence < self.get_type_non_prefix_precedence(&next_token) {
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
                    } else if name == "void" {
                        DataType::Void
                    } else {
                        return Err(format!("Unknown ident name : {:?}", name));
                    }
                };

                self.next(); // consumes ident
                return Ok(data_type);
            }

            Token::CurveOpenBracket => {
                let is_grouped_expression = {
                    let mut look_up_parser = self.lookup_parser();
                    while look_up_parser.get_cur_token()? != &Token::CurveCloseBracket {
                        look_up_parser.next();
                    }

                    look_up_parser.next(); // consumes )

                    if look_up_parser.get_cur_token()? == &Token::FunctionArrow {
                        false
                    } else {
                        true
                    }
                };

                if is_grouped_expression {
                    self.next(); // consumes (

                    let grouped_data_type = self.parse_type_declaration(1)?;

                    self.assert_cur_token(&Token::CurveCloseBracket)?;

                    self.next(); // consumes )
                    return Ok(grouped_data_type);
                } else {
                    self.next(); // consumes (

                    let mut arguments: IndexMap<String, DataType> = IndexMap::new();

                    let mut can_continue = true;

                    while self.get_cur_token()? != &Token::CurveCloseBracket && can_continue {
                        if let Token::Ident { name: arg_name } = self.get_cur_token()?.clone() {
                            self.next(); // consumes Ident

                            self.assert_cur_token(&Token::Colon)?;
                            self.next(); // consumes :

                            let arg_type = self.parse_type_declaration(1)?;

                            if arguments.contains_key(&arg_name) {
                                return Err(format!("In function declaration each argument must have different names but name : {} is repeated", arg_name));
                            } else {
                                arguments.insert(arg_name.to_string(), arg_type);
                            }

                            if let Token::Comma = self.get_cur_token()? {
                                self.next(); // consumes ,
                            } else {
                                can_continue = false;
                            }
                        } else {
                            return Err(format!(
                                "Expected tok to be Ident but got {:?}",
                                self.get_cur_token()?
                            ));
                        }
                    }

                    self.assert_cur_token(&Token::CurveCloseBracket)?;
                    self.next(); // consumes )

                    self.assert_cur_token(&Token::FunctionArrow)?;
                    self.next(); // consumes =>

                    let return_type = Box::new(self.parse_type_declaration(1)?);

                    return Ok(DataType::FunctionType {
                        arguments: convert_index_map_to_vec(&arguments),
                        return_type,
                    });
                }
            }

            Token::AngleOpenBracket => {
                self.next(); // consumes {

                let mut data_type_entries: IndexMap<String, DataType> = IndexMap::new();

                while self.get_cur_token()? != &Token::AngleCloseBracket {
                    if let Token::Ident { name } = self.get_cur_token()?.clone() {
                        self.next(); // consumes Ident;

                        self.assert_cur_token(&Token::Colon)?;
                        self.next();

                        let entry_data_type = self.parse_type_declaration(1)?;

                        data_type_entries.insert(name, entry_data_type);

                        if self.get_cur_token()? == &Token::Comma {
                            self.next(); // consume ,
                        } else {
                            self.assert_cur_token(&Token::AngleCloseBracket)?;
                        }
                    }
                }

                self.next(); // consumes }

                return Ok(DataType::ObjectType {
                    entries: data_type_entries,
                });
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
        // There is no non_prefix_type when left is Datatype::Void
        if left == DataType::Void {
            return Ok(Err(left));
        }

        let cur_tok = self.get_cur_token()?;

        match cur_tok {
            Token::BoxOpenBracket => {
                self.next(); // consumes [

                self.assert_cur_token(&Token::BoxCloseBracket)?;
                self.next(); // consumes ]
                return Ok(Ok(DataType::ArrayType {
                    base_type: Box::new(left),
                }));
            }

            _ => return Ok(Err(left)),
        }
    }

    pub(crate) fn _get_type_prefix_precedence(token: &Token) -> usize {
        match token {
            _ => 1,
        }
    }

    pub(crate) fn get_type_non_prefix_precedence(&self, token: &Token) -> usize {
        match token {
            Token::BoxOpenBracket => 5,
            _ => 1,
        }
    }
}
