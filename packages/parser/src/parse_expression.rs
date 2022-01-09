use std::collections::HashMap;

use ast::{data_type::DataType, expression::Expression};
use indexmap::IndexMap;
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

                assert_eq!(cur_tok, &Token::CurveCloseBracket);
                self.next(); // consumes )

                return Ok(grouped_exp);
            }

            // Parsing ArrayLiterals
            Token::BoxOpenBracket => {
                self.next(); // consumes [

                let mut expressions: Vec<Expression> = vec![];

                let mut there_is_comma = true;

                while self.get_cur_token()? != &Token::AngleCloseBracket && there_is_comma {
                    let item = self.parse_expression(1, context)?;

                    let tok = self.get_cur_token()?;

                    if tok == &Token::Comma {
                        there_is_comma = true;
                        self.next(); // consume ,
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

            // Assumes Parsing Object Literal
            Token::AngleOpenBracket => {
                self.next(); // consumes {

                let mut expression_entries: HashMap<String, Expression> = HashMap::new();
                let mut datatype_entries: IndexMap<String, DataType> = IndexMap::new();

                while self.get_cur_token()? != &Token::AngleCloseBracket {
                    if let Token::Ident { name } = self.get_cur_token()?.clone() {
                        self.next(); // consumes Ident

                        self.assert_cur_token(&Token::Colon)?;
                        self.next(); // consumes :

                        let exp = self.parse_expression(1, context)?;
                        let exp_data_type = exp.get_data_type();

                        expression_entries.insert(name.clone(), exp);
                        datatype_entries.insert(name.clone(), exp_data_type);

                        if self.get_cur_token()? == &Token::Comma {
                            self.next();
                        } else {
                            self.assert_cur_token(&Token::AngleCloseBracket)?;
                        }
                    } else {
                        return Err(format!(
                            "Expected token to be Ident but got {:?}",
                            self.get_cur_token()?
                        ));
                    }
                }

                self.next(); // consumes }
                return Ok(Expression::ObjectLiteral {
                    data_type: DataType::ObjectType {
                        entries: datatype_entries,
                    },
                    expression: expression_entries,
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

            Token::BoxOpenBracket => {
                self.next(); // consumes [

                let member_access_exp = self.parse_expression(1, context)?;

                self.assert_cur_token(&Token::BoxCloseBracket)?;
                self.next(); // consumes ]

                let data_type = member_access_exp.get_data_type();

                match data_type {
                    DataType::Float => {
                        let left_data_type = left.get_data_type();

                        if let DataType::ArrayType { base_type: _ } = left_data_type {
                            let exp = Expression::ArrayMemberAccess {
                                array: Box::new(left),
                                argument: Box::new(member_access_exp),
                            };

                            return Ok(Ok(exp));
                        } else {
                            return Err(format!("Expected the argument for left parameter to be expression of datatype ArrayType but got {:?}", left_data_type));
                        }
                    }

                    _ => {
                        return Err(format!(
                            "Expected datatype of member_access_exp to be Float but got {:?}",
                            data_type
                        ))
                    }
                }
            }

            Token::Dot => {
                self.next(); // consumes .

                if let Token::Ident { name } = self.get_cur_token()?.clone() {
                    let data_type = left.get_data_type();

                    if let DataType::ObjectType { entries: _ } = data_type {
                        self.next(); // consumes Ident

                        return Ok(Ok(Expression::DotMemberAccess {
                            container: Box::new(left),
                            argument: name.clone(),
                        }));
                    } else {
                        return Err(format!("Dot member access can be only used on expression whose datatype is ObjectType but used on data_type {:?}", data_type));
                    }
                } else {
                    return Err(format!(
                        "Expected token to be ident but got {:?}",
                        self.get_cur_token()?
                    ));
                }
            }

            Token::CurveOpenBracket => {
                self.next(); // consumes (

                let left_data_type = left.get_data_type();

                if let DataType::FunctionType {
                    arguments,
                    return_type,
                } = left_data_type
                {
                    let mut function_parameters: Vec<Expression> = vec![];
                    let mut index = 0;

                    while self.get_cur_token()?.clone() != Token::CurveCloseBracket {
                        let parameter = self.parse_expression(1, context)?;
                        let argument_index = arguments.get(index);

                        match argument_index {
                            None => return Err(format!("Function only takes only {} arguments but you are passing more than that", index)),

                            Some(data_type) => {
                                let parameter_data_type = parameter.get_data_type();

                                if parameter_data_type == *data_type {
                                    function_parameters.push(parameter);

                                    if let Token::Comma = self.get_cur_token()?.clone() {
                                        self.next(); // consumes ,
                                    } else {
                                        self.assert_cur_token(&Token::CurveCloseBracket)?;
                                    }

                                } else {
                                    return Err( format!("The datatype for {} argument is {:?} but got {:?}", index, data_type, parameter_data_type ));
                                }
                            }
                        }

                        index += 1;
                    }
                    self.next(); // consumes )

                    let fn_name = {
                        if let Expression::IdentExp { data_type: _, name } = &left {
                            name.clone()
                        } else {
                            return Err(format!("As of now only supported way for calling function is to use name of the function"));
                        }
                    };

                    return Ok(Ok(Expression::FunctionCall {
                        fn_name,
                        parameters: function_parameters,
                        return_type: return_type.as_ref().clone(),
                    }));
                } else {
                    return Err(format!("Can only use function call on expressions with datatype DataType::FunctionType not on datatype {:?}", left_data_type));
                }
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
            Token::BoxOpenBracket | Token::Dot | Token::CurveOpenBracket => 20,

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
