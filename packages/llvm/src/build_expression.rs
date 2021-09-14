use std::collections::HashMap;

use ast::{
    data_type::DataType,
    expression::{BinaryOperator, Expression, UnaryOperator},
};
use inkwell::{
    builder::Builder,
    context::Context,
    enums::{IntCompareOperator, RealCompareOperator},
    types::traits::BasicTypeTrait,
    values::{enums::BasicValueEnum, fn_value::FunctionValue, ptr_value::PointerValue},
};

pub(crate) fn build_expression<'a>(
    expression: &Expression,
    context: &'a Context,
    builder: &'a Builder,
    function_value: &mut FunctionValue,
    symbol_table: &mut HashMap<String, PointerValue<'a>>,
    name: Option<String>,
) -> BasicValueEnum<'a> {
    let name = match name {
        Some(name) => name,
        None => function_value.get_unique_reg_name(),
    };

    let name = name.as_str();

    match expression {
        Expression::FloatLiteralExp { name: _, value } => {
            let double_value = context.f64_type().const_float(*value);
            return BasicValueEnum::FloatValue(double_value);
        }

        Expression::BooleanLiteralExp { name: _, value } => {
            let bool_as_int_value = context.i1_type().const_int(*value as u64, false);
            return BasicValueEnum::IntValue(bool_as_int_value);
        }

        Expression::IdentExp {
            name: variable_name,
            data_type,
        } => {
            if let Some(pointer) = symbol_table.get(variable_name) {
                let load_value = match data_type {
                    DataType::Float => builder.build_load(
                        pointer.to_owned(),
                        context.f64_type().as_basic_type_enum(),
                        name,
                    ),
                    DataType::Boolean => builder.build_load(
                        pointer.to_owned(),
                        context.i1_type().as_basic_type_enum(),
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
            let arg_value = build_expression(
                argument.as_ref(),
                context,
                builder,
                function_value,
                symbol_table,
                None,
            );

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
                        UnaryOperator::Bang => {
                            builder.build_xor(value, context.i64_type().const_int(1, false), name)
                        }

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
            let left_value = build_expression(
                left.as_ref(),
                context,
                builder,
                function_value,
                symbol_table,
                None,
            );
            let right_value = build_expression(
                right.as_ref(),
                context,
                builder,
                function_value,
                symbol_table,
                None,
            );

            if let BasicValueEnum::FloatValue(lhs) = left_value {
                if let BasicValueEnum::FloatValue(rhs) = right_value {
                    let evaluated_float_value = match operator {
                        BinaryOperator::Plus => builder.build_float_add(lhs, rhs, name),
                        BinaryOperator::Minus => builder.build_float_sub(lhs, rhs, name),
                        BinaryOperator::Star => builder.build_float_mul(lhs, rhs, name),
                        BinaryOperator::Slash => builder.build_float_div(lhs, rhs, name),

                        BinaryOperator::StrictEquality => {
                            let int_value = match operator {
                                BinaryOperator::StrictEquality => builder.build_float_compare(
                                    RealCompareOperator::Equal,
                                    lhs,
                                    rhs,
                                    name,
                                ),

                                _ => unreachable!(),
                            };

                            return BasicValueEnum::IntValue(int_value);
                        }

                        _ => todo!(),
                    };

                    return BasicValueEnum::FloatValue(evaluated_float_value);
                } else {
                    todo!()
                }
            } else {
                if let BasicValueEnum::IntValue(lhs) = left_value {
                    if let BasicValueEnum::IntValue(rhs) = right_value {
                        let evaluated_int_value = match operator {
                            BinaryOperator::StrictEquality => {
                                builder.build_int_compare(IntCompareOperator::Equal, lhs, rhs, name)
                            }

                            _ => todo!(),
                        };

                        return BasicValueEnum::IntValue(evaluated_int_value);
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            }
        }

        _ => todo!(),
    }
}
