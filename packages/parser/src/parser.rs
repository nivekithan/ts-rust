use core::panic;

use ast::{
    data_type::DataType,
    declaration::{VariableAssignmentOperator, VariableDeclarationKind},
    Ast,
};
use lexer::token::{KeywordKind, Token};

use crate::symbol_table::{SymbolContext, SymbolMetaInsert};

pub struct Parser<'a> {
    pub(crate) content: &'a Vec<Token>,
    pub(crate) cur_pos: Option<usize>,
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

    pub fn next_ast(&mut self, global_context: &mut SymbolContext) -> Ast {
        return self.next_ast_in_context(global_context).unwrap();
    }

    pub(crate) fn next_ast_in_context(
        &mut self,
        context: &mut SymbolContext,
    ) -> Result<Ast, String> {
        let first_token = self.get_cur_token()?;
        let suffix = &context.suffix.clone();

        match first_token {
            Token::Keyword(keyword_kind) => match keyword_kind {
                KeywordKind::Const | KeywordKind::Let => {
                    let is_const = match keyword_kind {
                        KeywordKind::Const => true,
                        KeywordKind::Let => false,

                        _ => unreachable!(),
                    };

                    let kind = match keyword_kind {
                        KeywordKind::Const => VariableDeclarationKind::Const,
                        KeywordKind::Let => VariableDeclarationKind::Let,

                        _ => unreachable!(),
                    };

                    // let  name = format!("{}{}", self.next().get_ident_name()?.clone(), suffix); // consumes Const
                    let name = self.next().get_ident_name()?.clone(); // consumes const

                    self.next(); // consumes ident

                    let expected_data_type = match self.get_cur_token()? {
                        Token::Colon => {
                            self.next(); // consumes :
                            self.parse_type_declaration()
                        }

                        _ => DataType::Unknown,
                    };

                    self.assert_cur_token(&Token::Assign)?;

                    self.next(); // consumes =

                    let expression = self.parse_expression(1, context)?;

                    let expression_data_type = expression.get_data_type();

                    if expected_data_type != DataType::Unknown
                        && expected_data_type != expression_data_type
                    {
                        return Err(format!(
                            "Expected data type {:?} but got {:?}",
                            expected_data_type, expression_data_type
                        ));
                    }

                    let sym_meta = SymbolMetaInsert::create(expression_data_type, is_const);

                    if let Err(_) = context.insert(name.as_str(), sym_meta) {
                        return Err(format!(
                            "You cannot declare variable {} which is already declared",
                            name
                        ));
                    }

                    self.skip_semicolon()?;

                    let name_with_suffix = format!("{}{}", name, suffix);
                    return Ok(Ast::new_variable_declaration(
                        name_with_suffix.as_str(),
                        expression,
                        kind,
                    ));
                }

                KeywordKind::If => {
                    // let mut child_context = context.create_child_context();
                    let ast = self.parser_if_block(context)?;
                    return Ok(ast);
                }

                _ => {
                    return Err(format!(
                        "Update function next_ast\n Unexpected keyword, {:?}",
                        keyword_kind
                    ))
                }
            },

            Token::Ident { name } => {
                // let name = format!("{}{}",name.clone(), suffix);
                let name = name.clone();

                if let Some(sym_meta) = context.get(&name) {
                    if sym_meta.is_const {
                        return Err(format!("Cannot reassign a const variable"));
                    }

                    let data_type = sym_meta.data_type.clone();

                    self.next(); // consumes the ident

                    let operator = match self.get_cur_token()? {
                        Token::Assign => VariableAssignmentOperator::Assign,
                        Token::PlusAssign => VariableAssignmentOperator::PlusAssign,
                        Token::MinusAssign => VariableAssignmentOperator::MinusAssign,
                        Token::StarAssign => VariableAssignmentOperator::StarAssign,
                        Token::SlashAssign => VariableAssignmentOperator::SlashAssign,

                        tok => return Err(format!("Expected either one of the =, +=, -=, *=, /= assignment operators but got {:?}", tok)),
                    };

                    self.next(); // consumes =

                    let expression = self.parse_expression(1, context)?;

                    if expression.get_data_type() != data_type {
                        return Err(format!(
                            "Reassigning datatype {:?} to variable whose datatype is {:?}",
                            expression.get_data_type(),
                            data_type
                        ));
                    }

                    self.skip_semicolon()?;

                    let suffix_name = format!("{}{}", name, sym_meta.suffix);

                    return Ok(Ast::new_variable_assignment(
                        suffix_name.as_str(),
                        operator,
                        expression,
                    ));
                } else {
                    return Err(format!("Unknown variable {}", name));
                }
            }

            tok => return Err(format!("Unknown token: {:?}", tok)),
        }
    }

    pub(crate) fn parser_if_block(&mut self, context: &mut SymbolContext) -> Result<Ast, String> {
        let first_token = self.get_cur_token().unwrap();

        match first_token {
            Token::Keyword(KeywordKind::If) => {
                self.next(); // consumes if

                self.assert_cur_token(&Token::CurveOpenBracket)?;
                self.next(); // consumes (

                let condition = self.parse_expression(1, context)?;

                self.assert_cur_token(&Token::CurveCloseBracket)?;
                self.next(); // consumes )

                self.assert_cur_token(&Token::AngleOpenBracket)?;
                self.next(); // consumes {

                let cur_value = context.counter;
                let suffix = format!("{}{}", context.suffix, cur_value);
                context.counter += 1;

                let mut child_context = context.create_child_context(suffix);

                let mut if_block_ast: Vec<Ast> = vec![];

                while self.get_cur_token().unwrap() != &Token::AngleCloseBracket {
                    let ast = self.next_ast_in_context(&mut child_context)?;
                    if_block_ast.push(ast);
                }

                self.next(); // consumes }

                return Ok(Ast::new_if_block(condition, if_block_ast));
            }

            _ => panic!(
                "Expected parser_if_block to be called only when the cur_token is of Keyword if"
            ),
        }
    }

    pub(crate) fn next(&mut self) -> &Token {
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

    pub(crate) fn assert_cur_token(&self, token_type: &Token) -> Result<(), String> {
        let cur_token = self.get_cur_token()?;

        if cur_token != token_type {
            return Err(format!(
                "Expected token type to be {:?} but got {:?}",
                token_type, cur_token
            ));
        }

        return Ok(());
    }

    pub(crate) fn skip_semicolon(&mut self) -> Result<(), String> {
        let cur_token = self.get_cur_token()?;

        if let &Token::SemiColon = cur_token {
            self.next();
        }

        return Ok(());
    }

    pub(crate) fn get_cur_token(&self) -> Result<&Token, String> {
        if let Some(size) = self.cur_pos {
            return Ok(&self.content[size]);
        } else {
            return Err(
                "Next method should be called atleast one time before calling get_cur_token "
                    .to_string(),
            );
        }
    }

    pub(crate) fn parse_type_declaration(&mut self) -> DataType {
        let cur_tok = self.get_cur_token().unwrap();

        let data_type = match cur_tok {
            Token::Ident { name } => {
                if name == "string" {
                    DataType::String
                } else if name == "boolean" {
                    DataType::Boolean
                } else if name == "number" {
                    DataType::Float
                } else {
                    todo!()
                }
            }
            _ => todo!(),
        };

        self.next(); // consumes ident;

        return data_type;
    }
}
