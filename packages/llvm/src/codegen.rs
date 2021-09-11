use std::collections::HashMap;

use ast::{
    data_type::DataType,
    declaration::{Declaration, VariableAssignmentOperator},
    expression::{BinaryOperator, Expression, UnaryOperator},
    Ast,
};
use inkwell::{
    builder::Builder,
    context::Context,
    types::traits::BasicTypeTrait,
    values::{enums::BasicValueEnum, ptr_value::PointerValue},
};

#[derive(PartialEq, Eq)]
pub enum CodegenPos {
    Start,
    Pos(usize),
    End,
}

pub struct Codegen<'a> {
    content: &'a Vec<Ast>,
    cur_pos: CodegenPos,
    counter: usize,

    symbol_table: HashMap<String, PointerValue<'a>>,
}

impl<'a> Codegen<'a> {
    pub fn new(content: &Vec<Ast>) -> Codegen {
        return Codegen {
            content,
            cur_pos: CodegenPos::Start,
            counter: 0,

            symbol_table: HashMap::new(),
        };
    }

    fn get_temp_name(&mut self) -> String {
        let name = format!("{}", self.counter);
        self.counter += 1;
        return name;
    }

    pub(crate) fn consume(&mut self, context: &'a Context, builder: &'a Builder) {
        if let None = self.get_cur() {
            self.next();
        }

        while self.cur_pos != CodegenPos::End {
            if let Some(cur_ast) = self.get_cur() {
                match cur_ast {
                    Ast::Declaration(dec) => match dec {
                        Declaration::VariableDeclaration {
                            ident_name,
                            exp,
                            kind: _,
                        } => {
                            let data_type = exp.get_data_type();

                            let pointer = match data_type {
                                DataType::Float => {
                                    let pointer = builder
                                        .build_alloca(context.f64_type(), ident_name.as_str());
                                    let value_of_exp =
                                        self.build_expression(context, builder, exp, None);
                                    builder.build_store(pointer.clone(), value_of_exp);

                                    pointer
                                }

                                DataType::Boolean => {
                                    let pointer = builder
                                        .build_alloca(context.i64_type(), ident_name.as_str());
                                    let value_of_exp =
                                        self.build_expression(context, builder, exp, None);
                                    builder.build_store(pointer.clone(), value_of_exp);

                                    pointer
                                }

                                _ => todo!(),
                            };

                            self.symbol_table.insert(ident_name.to_owned(), pointer);
                        }

                        // Does not do typechecking
                        Declaration::VariableAssignment {
                            ident_name,
                            exp,
                            operator,
                        } => {
                            let var_ptr = self.symbol_table.get(ident_name).unwrap().clone();
                            let value = self.build_expression(context, builder, exp, None);

                            match operator {
                                VariableAssignmentOperator::Assign => {
                                    builder.build_store(var_ptr, value);
                                }

                                VariableAssignmentOperator::PlusAssign => {
                                    let load_value = builder.build_load(
                                        var_ptr,
                                        context.f64_type().as_basic_type_enum(),
                                        self.get_temp_name().as_str(),
                                    );

                                    if let BasicValueEnum::FloatValue(lhs) = load_value {
                                        if let BasicValueEnum::FloatValue(rhs) = value {
                                            let result_value = builder.build_float_add(
                                                lhs,
                                                rhs,
                                                self.get_temp_name().as_str(),
                                            );
                                            builder.build_store(var_ptr, result_value);
                                        }
                                    }
                                },

                                VariableAssignmentOperator::MinusAssign => {
                                    let load_value = builder.build_load(
                                        var_ptr,
                                        context.f64_type().as_basic_type_enum(),
                                        self.get_temp_name().as_str(),
                                    );

                                    if let BasicValueEnum::FloatValue(lhs) = load_value {
                                        if let BasicValueEnum::FloatValue(rhs) = value {
                                            let result_value = builder.build_float_sub(
                                                lhs,
                                                rhs,
                                                self.get_temp_name().as_str(),
                                            );
                                            builder.build_store(var_ptr, result_value);
                                        }
                                    }
                                },

                                
                                VariableAssignmentOperator::StarAssign => {
                                    let load_value = builder.build_load(
                                        var_ptr,
                                        context.f64_type().as_basic_type_enum(),
                                        self.get_temp_name().as_str(),
                                    );

                                    if let BasicValueEnum::FloatValue(lhs) = load_value {
                                        if let BasicValueEnum::FloatValue(rhs) = value {
                                            let result_value = builder.build_float_mul(
                                                lhs,
                                                rhs,
                                                self.get_temp_name().as_str(),
                                            );
                                            builder.build_store(var_ptr, result_value);
                                        }
                                    }
                                },

                                
                                VariableAssignmentOperator::SlashAssign => {
                                    let load_value = builder.build_load(
                                        var_ptr,
                                        context.f64_type().as_basic_type_enum(),
                                        self.get_temp_name().as_str(),
                                    );

                                    if let BasicValueEnum::FloatValue(lhs) = load_value {
                                        if let BasicValueEnum::FloatValue(rhs) = value {
                                            let result_value = builder.build_float_div(
                                                lhs,
                                                rhs,
                                                self.get_temp_name().as_str(),
                                            );
                                            builder.build_store(var_ptr, result_value);
                                        }
                                    }
                                }

                                // _ => todo!(),
                            };
                        }
                        _ => todo!(),
                    },

                    _ => todo!(),
                }
            } else {
                unreachable!()
            }

            self.next();
        }
        builder.build_return(None);
    }

    fn build_expression(
        &mut self,
        context: &'a Context,
        builder: &'a Builder,
        expression: &Expression,
        name: Option<String>,
    ) -> BasicValueEnum<'a> {
        let name = match name {
            Some(name) => name,
            None => self.get_temp_name(),
        };

        let name = name.as_str();

        match expression {
            Expression::FloatLiteralExp { name: _, value } => {
                let double_value = context.f64_type().const_float(*value);
                return BasicValueEnum::FloatValue(double_value);
            }

            Expression::BooleanLiteralExp { name: _, value } => {
                let bool_as_int_value = context.i64_type().const_int(*value as u64, false);
                return BasicValueEnum::IntValue(bool_as_int_value);
            }

            Expression::IdentExp {
                name: variable_name,
                data_type,
            } => {
                if let Some(pointer) = self.symbol_table.get(variable_name) {
                    let load_value = match data_type {
                        DataType::Float => builder.build_load(
                            pointer.to_owned(),
                            context.f64_type().as_basic_type_enum(),
                            name,
                        ),
                        DataType::Boolean => builder.build_load(
                            pointer.to_owned(),
                            context.i64_type().as_basic_type_enum(),
                            name,
                        ),

                        _ => todo!(),
                    };

                    return load_value;
                } else {
                    panic!("Unknown variable")
                }
            }

            Expression::UnaryExp { operator, argument } => {
                let arg_value = self.build_expression(context, builder, argument.as_ref(), None);

                match arg_value {
                    BasicValueEnum::FloatValue(value) => {
                        let evaluated_float_value = match operator {
                            UnaryOperator::Minus => builder.build_float_neg(value, name),
                            UnaryOperator::Plus => value,

                            _ => todo!(),
                        };

                        return BasicValueEnum::FloatValue(evaluated_float_value);
                    }

                    BasicValueEnum::IntValue(value) => {
                        let evaluated_int_value = match operator {
                            UnaryOperator::Bang => builder.build_xor(
                                value,
                                context.i64_type().const_int(1, false),
                                name,
                            ),

                            _ => todo!(),
                        };

                        return BasicValueEnum::IntValue(evaluated_int_value);
                    }

                    _ => todo!(),
                }
            }

            Expression::BinaryExp {
                operator,
                left,
                right,
            } => {
                let left_value = self.build_expression(context, builder, left.as_ref(), None);
                let right_value = self.build_expression(context, builder, right.as_ref(), None);

                if let BasicValueEnum::FloatValue(lhs) = left_value {
                    if let BasicValueEnum::FloatValue(rhs) = right_value {
                        let evaluated_float_value = match operator {
                            BinaryOperator::Plus => builder.build_float_add(lhs, rhs, name),
                            BinaryOperator::Minus => builder.build_float_sub(lhs, rhs, name),
                            BinaryOperator::Star => builder.build_float_mul(lhs, rhs, name),
                            BinaryOperator::Slash => builder.build_float_div(lhs, rhs, name),

                            _ => todo!(),
                        };

                        return BasicValueEnum::FloatValue(evaluated_float_value);
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            }

            _ => todo!(),
        }
    }
}

trait NextItem {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    fn get_cur(&self) -> Option<Self::Item>;
}

impl<'a> NextItem for Codegen<'a> {
    type Item = &'a Ast;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cur_pos {
            CodegenPos::Start => {
                self.cur_pos = CodegenPos::Pos(0);
                return Some(&self.content[0]);
            }
            CodegenPos::End => panic!("Calling next method after the content is consumed"),
            CodegenPos::Pos(pos) => {
                if pos >= self.content.len() - 1 {
                    self.cur_pos = CodegenPos::End;
                    return None;
                };

                self.cur_pos = CodegenPos::Pos(pos + 1);
                return Some(&self.content[pos + 1]);
            }
        }
    }

    fn get_cur(&self) -> Option<Self::Item> {
        match self.cur_pos {
            CodegenPos::Start => return None,
            CodegenPos::Pos(pos) => return Some(&self.content[pos]),
            CodegenPos::End => panic!("Calling get_cur method after the content is consumed"),
        }
    }
}
