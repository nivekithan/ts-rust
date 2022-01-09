use core::panic;

use ast::{
    data_type::DataType,
    declaration::{
        BlockWithCondition, Declaration, VariableAssignmentOperator, VariableDeclarationKind,
    },
    Ast,
};
use indexmap::IndexMap;
use lexer::token::{KeywordKind, Token};

use crate::{
    symbol_table::{FunctionSymbol, SymbolContext, SymbolMetaInsert},
    utils::convert_index_map_to_vec,
};

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

        match first_token {
            Token::Keyword(keyword_kind) => match keyword_kind {
                KeywordKind::Const | KeywordKind::Let => {
                    return self.parse_variable_declaration(context);
                }

                KeywordKind::If => {
                    let ast = self.parse_if_block(context)?;
                    return Ok(ast);
                }

                KeywordKind::While => {
                    let ast = self.parse_while_loop(context)?;
                    return Ok(ast);
                }

                KeywordKind::Do => {
                    let ast = self.parse_do_while_loop(context)?;
                    return Ok(ast);
                }

                KeywordKind::Break => {
                    self.next(); // consumes break
                    self.skip_semicolon()?;
                    return Ok(Ast::Declaration(Declaration::LoopControlFlow {
                        keyword: KeywordKind::Break,
                    }));
                }

                KeywordKind::Continue => {
                    self.next(); // consumes continue
                    self.skip_semicolon()?;
                    return Ok(Ast::Declaration(Declaration::LoopControlFlow {
                        keyword: KeywordKind::Continue,
                    }));
                }

                KeywordKind::Function => {
                    return self.parse_function_declaration(context);
                }

                KeywordKind::Return => {
                    self.next(); // consumes return

                    let return_exp = {
                        /*
                         * If token next to return is semicolon `;` then the
                         * function is returning void
                         * */
                        let cur_tok = self.get_cur_token()?;
                        if cur_tok == &Token::SemiColon {
                            self.next(); // consumes ;
                            None
                        } else {
                            let return_exp = self.parse_expression(1, context)?;
                            self.skip_semicolon()?; // consumes ;
                            Some(return_exp)
                        }
                    };

                    let expected_data_type = {
                        let return_type = context.get_return_type();
                        match return_type {
                            None => {
                                return Err(format!(
                                    "Cannot use return keyword outside of function declaration"
                                ))
                            }
                            Some(d) => d,
                        }
                    };

                    let actual_data_type = {
                        if let Some(exp) = &return_exp {
                            exp.get_data_type()
                        } else {
                            DataType::Void
                        }
                    };

                    if expected_data_type != &actual_data_type {
                        return Err(format!("Expected the return_type declared in function declaration does not match the data_type of expression which the function is returning"));
                    }

                    self.skip_semicolon()?;
                    return Ok(Ast::new_return_statement(return_exp));
                }

                _ => {
                    return Err(format!(
                        "Update function next_ast\n Unexpected keyword, {:?}",
                        keyword_kind
                    ))
                }
            },

            Token::Ident { name } => {
                if let Some(_) = context.get(name) {
                    return self.parse_naked_ident(context);
                } else {
                    return Err(format!("Unknown variable {}", name));
                }
            }

            _ => return self.parse_naked_expression(context),
        }
    }

    /*
     * Assumes the current token to be 'keyword if' in
     *
     *   if (<condition>) {
     *      <block>
     *   } .....
     *
     * Consumes till token '}' in
     *
     * if (<condition>) {
     *      <block>
     * }
     *
     * Pass current scope context no need to create child context
     *
     * */
    pub(crate) fn parse_if_block(&mut self, context: &mut SymbolContext) -> Result<Ast, String> {
        let cur_tok = self.get_cur_token()?;

        match cur_tok {
            Token::Keyword(KeywordKind::If) => {
                self.next(); // consumes if

                self.assert_cur_token(&Token::CurveOpenBracket)?;
                let if_block = self.parse_block_with_condition(context)?;
                let mut else_if_block: Vec<BlockWithCondition> = vec![];
                let mut else_block: Option<Box<Vec<Ast>>> = None;

                loop {
                    let cur_tok = self.get_cur_token()?;

                    match cur_tok {
                        Token::Keyword(KeywordKind::Else) => {
                            self.next(); // consumes keyword else

                            let cur_tok = self.get_cur_token()?;

                            match cur_tok {
                                Token::Keyword(KeywordKind::If) => {
                                    self.next(); // consumes keyword if
                                    let single_else_if_block = self.parse_block_with_condition(context)?;
                                    else_if_block.push(single_else_if_block);
                                    continue;
                                },

                                Token::AngleOpenBracket => {
                                    let ast_block = self.parse_block(context)?;
                                    else_block = Some(Box::new(ast_block));
                                    return Ok(Ast::new_if_block(if_block, else_if_block, else_block));

                                },

                                tok => return Err(format!("Expected token to be either keyword if or token {{ but got token {:?}", tok)) 
                            }
                        }

                        _ => return Ok(Ast::new_if_block(if_block, else_if_block, else_block)),
                    }
                }
            }

            _ => panic!(
                "Expected parser_if_block to be called only when the cur_token is of Keyword if"
            ),
        }
    }

    /*
     * Assumes the current token to be `keyword while` in
     *
     * while (<condition>) {
     *     <block>
     * }
     *
     * Consumes till token `}` in
     *
     * while (<condition>) {
     *     <block>
     * }
     *
     * Pass the current scope context no need to create child context
     *
     * */
    pub(crate) fn parse_while_loop(&mut self, context: &mut SymbolContext) -> Result<Ast, String> {
        self.assert_cur_token(&Token::Keyword(KeywordKind::While))?;

        self.next(); // consumes while

        let block_with_condition = self.parse_block_with_condition(context)?;
        return Ok(Ast::new_while_loop(block_with_condition));
    }

    pub(crate) fn parse_do_while_loop(
        &mut self,
        context: &mut SymbolContext,
    ) -> Result<Ast, String> {
        self.assert_cur_token(&Token::Keyword(KeywordKind::Do))?;

        self.next(); // consumes do

        let block = self.parse_block(context)?;

        self.assert_cur_token(&Token::Keyword(KeywordKind::While))?;
        self.next(); // consumes while

        self.assert_cur_token(&Token::CurveOpenBracket)?;
        self.next(); // consumes (

        let condition = self.parse_expression(1, context)?;

        self.assert_cur_token(&Token::CurveCloseBracket)?;
        self.next(); // consumes )

        let block_with_condition = BlockWithCondition {
            block: Box::new(block),
            condition,
        };
        return Ok(Ast::new_do_while_loop(block_with_condition));
    }

    pub(crate) fn parse_variable_declaration(
        &mut self,
        context: &mut SymbolContext,
    ) -> Result<Ast, String> {
        let cur_tok = self.get_cur_token()?;
        let suffix = &context.suffix.clone();

        match cur_tok {
            Token::Keyword(keyword_kind) => {
                match keyword_kind {
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
                                let data_type = self.parse_type_declaration(1)?;

                                if data_type == DataType::Void {
                                    return Err(format!("Void type can be only used as return type in function but ident {:?} is explicitly declared as void", name));
                                }

                                data_type
                            }

                            _ => DataType::Unknown,
                        };

                        self.assert_cur_token(&Token::Assign)?;

                        self.next(); // consumes =

                        let expression = self.parse_expression(1, context)?;

                        let expression_data_type = expression.get_data_type();

                        /*
                         * We cannot allow following type of code
                         *
                         * ```
                         *  function foo(x : number) : void {
                         *  return;
                         * }
                         *
                         * const x = foo(1);
                         *
                         * ```
                         *
                         * Where a variable x is assigned to return_value of fn whose return_type is void
                         *
                         * In future when we add support for `undefined` datatype we can allow this
                         *
                         *  */
                        if expression_data_type == DataType::Void {
                            return Err(format!(
                                "Cannot assign expression with datatype as void to variable {:?}",
                                name
                            ));
                        }

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

                    k => {
                        return Err(format!(
                        "Expected to be token keyword Const or keyword true but got keyword {:?}",
                        k
                    ))
                    }
                }
            }

            tok => {
                return Err(format!(
                    "Expected to be token keyword const or keyword true but got token {:?}",
                    tok
                ))
            }
        }
    }

    /*
     * While parsing if we come across a ident which is in context (i.e it is defined before) then
     * either that is variable reassignment or some expression whose returned value is not
     * assigned to some variable
     *
     * This function will check what case that is and pass the control flow to either
     * parse_variable_assignment() or parse_naked_expression()
     *
     * I could not come up with a function name that explains its function
     *
     **/
    pub(crate) fn parse_naked_ident(&mut self, context: &mut SymbolContext) -> Result<Ast, String> {
        let cur_tok = self.get_cur_token()?;
        let mut lookup_parser = self.clone();
        match cur_tok {
            Token::Ident { name: _ } => 'outer: loop {
                let next_tok = lookup_parser.next();

                if VariableAssignmentOperator::is_lexer_assignment_operator(next_tok) {
                    return self.parse_variable_assignment(context);
                }

                if next_tok == &Token::BoxOpenBracket {
                    while lookup_parser.next() != &Token::BoxCloseBracket {
                        continue;
                    }

                    continue 'outer;
                }

                return self.parse_naked_expression(context);
            },

            tok => return Err(format!("Expected tok to be of ident but got {:?}", tok)),
        }
    }

    /*
     * This function parses the expression and binds the expression value to a temporary
     * variable that is code like below will be converted to
     *
     * ```
     * 5 + 5
     *
     * foo();
     * ```
     * something like this
     *
     * ```
     * const |temp1 = 5 + 5;
     * const |temp2 = foo();
     * ```
     *
     * Since variable in typescript cannot begin with '|' it wont lead to any
     * variable name conflict
     *
     * TODO:
     *
     *    -> Hopefully in future when can add code that will ignore expression that has no
     * sideeffect
     *
     **/

    pub(crate) fn parse_naked_expression(
        &mut self,
        context: &mut SymbolContext,
    ) -> Result<Ast, String> {
        let exp = self.parse_expression(1, context)?;
        self.skip_semicolon()?;
        let name = context.get_temp_name();

        let sym_meta = SymbolMetaInsert::create(exp.get_data_type(), true);

        if let Err(_) = context.insert(name.as_str(), sym_meta) {
            return Err(format!(
                "[INTERNAL ERROR](parser.parse_naked_expression) There is already a temp variable with name {}",
                name
            ));
        }

        self.skip_semicolon()?;

        // let name_with_suffix = format!("{}{}", name, suffix);
        return Ok(Ast::new_variable_declaration(
            name.as_str(),
            exp,
            VariableDeclarationKind::Const,
        ));
    }

    pub(crate) fn parse_variable_assignment(
        &mut self,
        context: &SymbolContext,
    ) -> Result<Ast, String> {
        let cur_tok = &self.get_cur_token()?.clone();

        match cur_tok {
            Token::Ident { name } => {
                let sym_meta = context.get(name).unwrap();

                self.next(); // consumes the ident

                let next_token = self.get_cur_token()?;

                match next_token {
                    Token::BoxOpenBracket => {
                        let array_datatype = &sym_meta.data_type;

                        if let DataType::ArrayType { base_type } = array_datatype {
                            self.next(); // consumes [

                            let member_access_exp = self.parse_expression(1, context)?;

                            if let DataType::Float = member_access_exp.get_data_type() {
                            } else {
                                return Err(format!("Expected data_type of array member access expression to be float but got {:?}", member_access_exp.get_data_type()));
                            }

                            self.assert_cur_token(&Token::BoxCloseBracket)?;
                            self.next(); // consumes ]

                            let operator = match self.get_cur_token()? {
                            Token::Assign => VariableAssignmentOperator::Assign,
                            Token::PlusAssign => VariableAssignmentOperator::PlusAssign,
                            Token::MinusAssign => VariableAssignmentOperator::MinusAssign,
                            Token::StarAssign => VariableAssignmentOperator::StarAssign,
                            Token::SlashAssign => VariableAssignmentOperator::SlashAssign,
                            tok => return Err(format!("Expected either one of the =, +=, -=, *=, /= assignment operators but got {:?}", tok)),
                        };

                            self.next(); // consumes VariableAssignmentOperator

                            let exp = self.parse_expression(1, context)?;

                            if &exp.get_data_type() != base_type.as_ref() {
                                return Err(format!(
                                    "Reassigning datatype {:?} to variable whose datatype is {:?}",
                                    exp.get_data_type(),
                                    base_type
                                ));
                            }

                            self.skip_semicolon()?;

                            let suffix_name = format!("{}{}", name, sym_meta.suffix);

                            return Ok(Ast::new_array_member_assignment(
                                suffix_name.as_str(),
                                member_access_exp,
                                operator,
                                exp,
                            ));
                        } else {
                            return Err(format!(
                                "Expected the datatype of ident {:?} to be ArrayType but got {:?}",
                                name, array_datatype
                            ));
                        }
                    }

                    _ => {
                        if sym_meta.is_const {
                            return Err(format!("Cannot reassign a const variable"));
                        }

                        let data_type = sym_meta.data_type.clone();

                        let operator = match self.get_cur_token()? {
                            Token::Assign => VariableAssignmentOperator::Assign,
                            Token::PlusAssign => VariableAssignmentOperator::PlusAssign,
                            Token::MinusAssign => VariableAssignmentOperator::MinusAssign,
                            Token::StarAssign => VariableAssignmentOperator::StarAssign,
                            Token::SlashAssign => VariableAssignmentOperator::SlashAssign,
                            tok => return Err(format!("Expected either one of the =, +=, -=, *=, /= assignment operators but got {:?}", tok)),
                        };

                        self.next(); // consumes VariableAssignmentOperator

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
                    }
                }
            }

            tok => return Err(format!("Expected tok to be of ident but got {:?}", tok)),
        }
    }

    /*
     * Assumes the current token to be `keyword functions` in
     *
     * function name(parameter1 : type1, parameter2 : type2 ) : returnType {
     *      <block>
     * };
     *
     * Expects the returnType to be explicitly defined
     *
     * Consumes till the token ;
     *
     * Pass the context no need to create child context
     *
     * TODO: Support closures
     *
     * */
    pub(crate) fn parse_function_declaration(
        &mut self,
        context: &mut SymbolContext,
    ) -> Result<Ast, String> {
        self.assert_cur_token(&Token::Keyword(KeywordKind::Function))?;
        self.next(); // consumes keyword function

        if let Token::Ident { name } = self.get_cur_token()?.clone() {
            self.next(); // consumes Ident

            self.assert_cur_token(&Token::CurveOpenBracket)?;
            self.next(); // consumes (

            let mut arguments: IndexMap<String, DataType> = IndexMap::new();

            while self.get_cur_token()?.clone() != Token::CurveCloseBracket {
                if let Token::Ident { name } = self.get_cur_token()?.clone() {
                    self.next(); // consumes Ident

                    self.assert_cur_token(&Token::Colon)?;
                    self.next(); // consumes :

                    let data_type = self.parse_type_declaration(1)?;
                    let name = format!("{}{}", name, context.suffix);
                    if arguments.contains_key(&name) {
                        return Err(format!("In function declaration each argument must have different names but name : {} is repeated", &name));
                    } else {
                        arguments.insert(name, data_type);
                    }

                    if let Token::Comma = self.get_cur_token()?.clone() {
                        self.next();
                        continue;
                    } else {
                        self.assert_cur_token(&Token::CurveCloseBracket)?;
                    }
                } else {
                    return Err(format!(
                        "Expected current token to be of Ident but got {:?}",
                        self.get_cur_token()?
                    ));
                }
            }

            self.next(); // consumes )

            self.assert_cur_token(&Token::Colon)?;
            self.next(); // consumes :

            let return_type = self.parse_type_declaration(1)?;

            self.assert_cur_token(&Token::AngleOpenBracket)?;

            let function_block_context = &mut SymbolContext::create_function_context(
                FunctionSymbol::new(return_type.clone()),
            );

            for (arg_name, arg_data_type) in &arguments {
                /*
                 * In arguments the arg_name are with the suffix but to insert into
                 * function_block_context the name should not have suffix as it will be
                 * added by the context itself
                 *
                 * So we have to remove the suffix
                 * */

                let arg_name_cloned = arg_name.clone();
                let arg_name_without_suffix =
                    arg_name_cloned.strip_suffix(&context.suffix).unwrap();

                let sym_meta = SymbolMetaInsert::create(arg_data_type.clone(), false);
                function_block_context.insert(arg_name_without_suffix, sym_meta)?;
            }

            let block = self.parse_block_with_context(function_block_context)?;

            context.insert(
                name.as_str(),
                SymbolMetaInsert::create(
                    DataType::FunctionType {
                        arguments: convert_index_map_to_vec(&arguments),
                        return_type: Box::new(return_type.clone()),
                    },
                    true,
                ),
            )?;

            self.skip_semicolon()?;
            let name_with_suffix = format!("{}{}", name, context.suffix.clone());
            return Ok(Ast::new_function_declaration(
                arguments,
                Box::new(block),
                name_with_suffix,
                return_type,
            ));
        } else {
            return Err(format!(
                "Expected the current token to be ident but got {:?}",
                self.get_cur_token()?
            ));
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

    pub(crate) fn clone(&self) -> Parser<'a> {
        return Parser {
            content: self.content,
            cur_pos: self.cur_pos,
        };
    }

    // pub(crate) fn lookup_next(&self) -> Option<&Token> {
    //     return self.lookup_with_offset(1);
    // }

    // pub(crate) fn lookup_with_offset(&self, offset: usize) -> Option<&Token> {
    //     match self.cur_pos {
    //         None => {
    //             let value = offset - 1;
    //             return Some(&self.content[value]);
    //         }

    //         Some(cur_pos) => {
    //             let value = cur_pos + offset;

    //             if value > self.content.len() - 1 {
    //                 return None;
    //             }

    //             return Some(&self.content[value]);
    //         }
    //     }
    // }

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
}
