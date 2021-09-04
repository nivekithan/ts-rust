use ast::{
    data_type::DataType,
    declaration::Declaration,
    expression::{BinaryOperator, Expression, UnaryOperator},
    Ast,
};
use inkwell::{builder::Builder, context::Context, values::enums::BasicValueEnum};

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
}

impl<'a> Codegen<'a> {
    pub fn new(content: &Vec<Ast>) -> Codegen {
        return Codegen {
            content,
            cur_pos: CodegenPos::Start,
            counter: 0,
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
                        Declaration::ConstVariableDeclaration { ident_name, exp } => {
                            let data_type = exp.get_data_type();

                            match data_type {
                                DataType::Float => {
                                    let pointer = builder
                                        .build_alloca(context.f64_type(), ident_name.as_str());
                                    let value_of_exp =
                                        self.build_expresserion(context, builder, exp, None);
                                    builder.build_store(pointer, value_of_exp);
                                }

                                DataType::Boolean => {
                                    let pointer = builder
                                        .build_alloca(context.i64_type(), ident_name.as_str());
                                    let value_of_exp =
                                        self.build_expresserion(context, builder, exp, None);
                                    builder.build_store(pointer, value_of_exp);
                                }

                                _ => todo!(),
                            }
                        }
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

    fn build_expresserion(
        &mut self,
        context: &'a Context,
        builder: &'a Builder,
        expression: &Expression,
        name: Option<String>,
    ) -> BasicValueEnum<'a> {
        match expression {
            Expression::FloatLiteralExp { name: _, value } => {
                let double_value = context.f64_type().const_float(*value);
                return BasicValueEnum::FloatValue(double_value);
            }

            Expression::BooleanLiteralExp { name: _, value } => {
                let bool_as_int_value = context.i64_type().const_int(*value as u64, false);
                return BasicValueEnum::IntValue(bool_as_int_value);
            }

            Expression::UnaryExp { operator, argument } => {
                let arg_value = self.build_expresserion(context, builder, argument.as_ref(), None);

                let name = match name {
                    Some(name) => name,
                    None => self.get_temp_name(),
                };

                let name = name.as_str();

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
                let left_value = self.build_expresserion(context, builder, left.as_ref(), None);
                let right_value = self.build_expresserion(context, builder, right.as_ref(), None);

                if let BasicValueEnum::FloatValue(lhs) = left_value {
                    if let BasicValueEnum::FloatValue(rhs) = right_value {
                        let name = match name {
                            Some(name) => name,
                            None => self.get_temp_name(),
                        };

                        let name = name.as_str();

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