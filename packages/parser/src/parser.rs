use core::panic;

use ast::{
    data_type::DataType,
    declaration::{VariableAssignmentOperator, VariableDeclarationKind},
    expression::{BinaryOperator, Expression, UnaryOperator},
    Ast,
};
use lexer::token::{KeywordKind, LiteralKind, Token};

use crate::symbol_table::{SymbolContext, SymbolMetaInsert};

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

    pub fn next_ast(&mut self, global_context: &mut SymbolContext) -> Ast {
        return self.next_ast_in_context(global_context).unwrap();
    }


    fn next_ast_in_context(&mut self, context: &mut SymbolContext) -> Result<Ast, String> {
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

    fn parser_if_block(&mut self, context: &mut SymbolContext) -> Result<Ast, String> {
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

                let mut if_block_ast : Vec<Ast> = vec![];

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

    fn assert_cur_token(&self, token_type: &Token) -> Result<(), String> {
        let cur_token = self.get_cur_token()?;

        if cur_token != token_type {
            return Err(format!(
                "Expected token type to be {:?} but got {:?}",
                token_type, cur_token
            ));
        }

        return Ok(());
    }

    fn skip_semicolon(&mut self) -> Result<(), String> {
        let cur_token = self.get_cur_token()?;

        if let &Token::SemiColon = cur_token {
            self.next();
        }

        return Ok(());
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

    fn parse_expression(
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

    fn get_prefix_exp(&mut self, context: &SymbolContext) -> Result<Expression, String> {
        let cur_token = self.get_cur_token().unwrap();

        match cur_token {
            Token::Plus => {
                let precedence = Parser::get_prefix_precedence(&Token::Plus);

                self.next(); // consumes +
                let arg_exp = self.parse_expression(precedence, context)?;
                return Ok(Expression::UnaryExp {
                    operator: UnaryOperator::Plus,
                    argument: Box::new(arg_exp),
                });
            }

            Token::Minus => {
                let precedence = Parser::get_prefix_precedence(&Token::Minus);

                self.next(); // consumes -
                let arg_exp = self.parse_expression(precedence, context)?;
                return Ok(Expression::UnaryExp {
                    operator: UnaryOperator::Minus,
                    argument: Box::new(arg_exp),
                });
            }

            Token::Bang => {
                let precedence = Parser::get_prefix_precedence(&Token::Bang);

                self.next(); //  consumes !
                let arg_exp = self.parse_expression(precedence, context)?;
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

            tok => {
                return Err(format!(
                    "Given token {:?} does not have not a prefix function",
                    tok
                ))
            }
        }
    }

    fn get_non_prefix_exp(
        &mut self,
        left: Expression,
        context: &SymbolContext,
    ) -> Result<Result<Expression, Expression>, String> {
        let non_prefix_token = self.get_cur_token().unwrap();

        match non_prefix_token {
            Token::Plus => {
                let precedence = Parser::get_non_prefix_precedence(&Token::Plus);

                self.next(); // consumes +

                let right_exp = Box::new(self.parse_expression(precedence, context)?);
                return Ok(Ok(Expression::BinaryExp {
                    operator: BinaryOperator::Plus,
                    left: Box::new(left),
                    right: right_exp,
                }));
            }

            Token::Minus => {
                let precedence = Parser::get_non_prefix_precedence(&Token::Minus);

                self.next(); // consumes -

                let right_exp = Box::new(self.parse_expression(precedence, context)?);
                return Ok(Ok(Expression::BinaryExp {
                    operator: BinaryOperator::Minus,
                    left: Box::new(left),
                    right: right_exp,
                }));
            }

            Token::Star => {
                let precedence = Parser::get_non_prefix_precedence(&Token::Star);

                self.next(); // consumes *

                let right_exp = Box::new(self.parse_expression(precedence, context)?);
                return Ok(Ok(Expression::BinaryExp {
                    operator: BinaryOperator::Star,
                    left: Box::new(left),
                    right: right_exp,
                }));
            }

            Token::Slash => {
                let precedence = Parser::get_non_prefix_precedence(&Token::Slash);

                self.next(); // consumes /

                let right_exp = Box::new(self.parse_expression(precedence, context)?);
                return Ok(Ok(Expression::BinaryExp {
                    operator: BinaryOperator::Slash,
                    left: Box::new(left),
                    right: right_exp,
                }));
            }

            Token::VerticalBar => {
                let precedence = Parser::get_non_prefix_precedence(&Token::VerticalBar);

                self.next(); // consumes |

                let right_exp = Box::new(self.parse_expression(precedence, context)?);
                return Ok(Ok(Expression::BinaryExp {
                    operator: BinaryOperator::VerticalBar,
                    left: Box::new(left),
                    right: right_exp,
                }));
            }

            Token::Caret => {
                let precedence = Parser::get_non_prefix_precedence(&Token::Caret);

                self.next(); // consumes ^

                let right_exp = Box::new(self.parse_expression(precedence, context)?);
                return Ok(Ok(Expression::BinaryExp {
                    operator: BinaryOperator::Caret,
                    left: Box::new(left),
                    right: right_exp,
                }));
            }

            Token::Ampersand => {
                let precedence = Parser::get_non_prefix_precedence(&Token::Ampersand);

                self.next(); // consumes &

                let right_exp = Box::new(self.parse_expression(precedence, context)?);
                return Ok(Ok(Expression::BinaryExp {
                    operator: BinaryOperator::Ampersand,
                    left: Box::new(left),
                    right: right_exp,
                }));
            }

            Token::StrictEquality => {
                let precedence = Parser::get_non_prefix_precedence(&Token::StrictEquality);

                self.next(); // consumes ===

                let right_exp = Box::new(self.parse_expression(precedence, context)?);
                return Ok(Ok(Expression::BinaryExp {
                    operator: BinaryOperator::StrictEquality,
                    left: Box::new(left),
                    right: right_exp,
                }));
            }

            Token::StrictNotEqual => {
                let precedence = Parser::get_non_prefix_precedence(&Token::StrictNotEqual);

                self.next(); // consumes !==

                let right_exp = Box::new(self.parse_expression(precedence, context)?);
                return Ok(Ok(Expression::BinaryExp {
                    operator: BinaryOperator::StrictNotEqual,
                    left: Box::new(left),
                    right: right_exp,
                }));
            }

            Token::LessThan => {
                let precedence = Parser::get_non_prefix_precedence(&Token::LessThan);

                self.next(); // consumes !==

                let right_exp = Box::new(self.parse_expression(precedence, context)?);
                return Ok(Ok(Expression::BinaryExp {
                    operator: BinaryOperator::LessThan,
                    left: Box::new(left),
                    right: right_exp,
                }));
            }

            Token::LessThanOrEqual => {
                let precedence = Parser::get_non_prefix_precedence(&Token::LessThanOrEqual);

                self.next(); // consumes !==

                let right_exp = Box::new(self.parse_expression(precedence, context)?);
                return Ok(Ok(Expression::BinaryExp {
                    operator: BinaryOperator::LessThanOrEqual,
                    left: Box::new(left),
                    right: right_exp,
                }));
            }

            Token::GreaterThan => {
                let precedence = Parser::get_non_prefix_precedence(&Token::GreaterThan);

                self.next(); // consumes !==

                let right_exp = Box::new(self.parse_expression(precedence, context)?);
                return Ok(Ok(Expression::BinaryExp {
                    operator: BinaryOperator::GreaterThan,
                    left: Box::new(left),
                    right: right_exp,
                }));
            }

            Token::GreaterThanOrEqual => {
                let precedence = Parser::get_non_prefix_precedence(&Token::GreaterThanOrEqual);

                self.next(); // consumes !==

                let right_exp = Box::new(self.parse_expression(precedence, context)?);
                return Ok(Ok(Expression::BinaryExp {
                    operator: BinaryOperator::GreaterThanOrEqual,
                    left: Box::new(left),
                    right: right_exp,
                }));
            }

            _ => return Ok(Err(left)),
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

    fn parse_type_declaration(&mut self) -> DataType {
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
