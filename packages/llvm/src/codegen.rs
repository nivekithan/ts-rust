use ast::{
    data_type::DataType,
    declaration::Declaration,
    expression::{BinaryOperator, Expression},
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

            Expression::BinaryExp {
                operator,
                left,
                right,
            } => match operator {
                // BinaryOperator::Plus => {
                //     let left_value = self.build_expresserion(context, builder, left.as_ref(), None);
                //     let right_value =
                //         self.build_expresserion(context, builder, right.as_ref(), None);

                //     if let BasicValueEnum::FloatValue(left_float) = left_value {
                //         if let BasicValueEnum::FloatValue(right_float) = right_value {
                //             let exp_name = match name {
                //                 Some(name) => name,
                //                 None => self.get_temp_name(),
                //             };

                //             let added_float_value =
                //                 builder.build_float_add(left_float, right_float, exp_name.as_str());

                //             return BasicValueEnum::FloatValue(added_float_value);
                //         } else {
                //             todo!()
                //         }
                //     } else {
                //         todo!()
                //     }
                // },

                // BinaryOperator::Minus => {
                // let left_value = self.build_expresserion(context, builder, left.as_ref(), None);
                // let right_value = self.build_expresserion(context, builder, right.as_ref(), None);

                // if let BasicValueEnum::FloatValue(left_float) = left_value {
                //     if let BasicValueEnum::FloatValue(right_float) = right_value {
                //         let exp_name = match name {
                //             Some(name) => name,
                //             None => self.get_temp_name(),
                //         };

                //         let subbed_float = builder.build_float_sub(left_float, right_float, exp_name.as_str());

                //         return BasicValueEnum::FloatValue(subbed_float);
                //     } else {
                //         todo!()
                //     }
                //     } else {
                //         todo!()
                //     }
                // }
                operator => {
                    let left_value = self.build_expresserion(context, builder, left.as_ref(), None);
                    let right_value =
                        self.build_expresserion(context, builder, right.as_ref(), None);

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
            },

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
