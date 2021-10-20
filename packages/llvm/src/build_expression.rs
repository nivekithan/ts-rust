use std::{collections::HashMap, convert::TryInto};

use ast::{
    data_type::DataType,
    expression::{BinaryOperator, Expression, UnaryOperator},
};
use inkwell::{builder::Builder, context::Context, enums::{IntCompareOperator, RealCompareOperator}, types::{array_type::ArrayType, traits::BasicTypeTrait}, values::{enums::BasicValueEnum, fn_value::FunctionValue, ptr_value::PointerValue}};

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

        Expression::StringLiteralExp { value } => {
            let size_of_string = value.len() as u32;
            let string_array_type = context.i8_type().array_type(size_of_string);

            let base_pointer = builder.build_alloca(string_array_type, name);

            for (i, c) in value.chars().enumerate() {
                let indices = vec![
                    context.i64_type().const_int(0, true),
                    context.i64_type().const_int(i.try_into().unwrap(), true),
                ];

                let index_pointer = builder.build_gep_2(
                    string_array_type,
                    &base_pointer,
                    &indices,
                    &function_value.get_unique_reg_name(),
                );

                let char_value = context.i8_type().const_int(c as u64, false);
                builder.build_store(index_pointer, char_value);
            }

            return BasicValueEnum::PointerValue(base_pointer);
        }

        /*
         * param 'name' is not used if the datatype of IdentExp is one of
         *          - String
         *          - ArrayType
         *
         *
         **/
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
                    DataType::String => BasicValueEnum::PointerValue(pointer.clone()),

                    DataType::ArrayType{base_type : _ } => BasicValueEnum::PointerValue(pointer.clone()),

                    _ => panic!("Update Function build_expression -> Expression::IdentExp, Unsupported datatype"),
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

                        BinaryOperator::StrictEquality
                        | BinaryOperator::StrictNotEqual
                        | BinaryOperator::LessThan
                        | BinaryOperator::LessThanOrEqual
                        | BinaryOperator::GreaterThan
                        | BinaryOperator::GreaterThanOrEqual => {
                            let int_value = match operator {
                                BinaryOperator::StrictEquality => builder.build_float_compare(
                                    RealCompareOperator::Equal,
                                    lhs,
                                    rhs,
                                    name,
                                ),

                                BinaryOperator::StrictNotEqual => builder.build_float_compare(
                                    RealCompareOperator::NotEqual,
                                    lhs,
                                    rhs,
                                    name,
                                ),

                                BinaryOperator::LessThan => builder.build_float_compare(
                                    RealCompareOperator::LessThan,
                                    lhs,
                                    rhs,
                                    name,
                                ),

                                BinaryOperator::LessThanOrEqual => builder.build_float_compare(
                                    RealCompareOperator::LessThanOrEqual,
                                    lhs,
                                    rhs,
                                    name,
                                ),

                                BinaryOperator::GreaterThan => builder.build_float_compare(
                                    RealCompareOperator::GreaterThan,
                                    lhs,
                                    rhs,
                                    name,
                                ),

                                BinaryOperator::GreaterThanOrEqual => builder.build_float_compare(
                                    RealCompareOperator::GreaterThanOrEqual,
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
                            BinaryOperator::StrictNotEqual => builder.build_int_compare(
                                IntCompareOperator::NotEqual,
                                lhs,
                                rhs,
                                name,
                            ),

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

        Expression::ArrayLiteral {
            expression,
            expression_data_type,
        } => {
            let array_type = convert_data_type_to_array_type(
                expression_data_type,
                context,
                expression.len() as u32,
            )
            .unwrap();

            let base_pointer = builder.build_alloca(array_type, name);

            for (i, exp) in expression.iter().enumerate() {
                let value =
                    build_expression(exp, context, builder, function_value, symbol_table, None);

                let indices = vec![
                    context.i64_type().const_int(0, true),
                    context.i64_type().const_int(i.try_into().unwrap(), true),
                ];

                let index_pointer = builder.build_gep_2(
                    array_type,
                    &base_pointer,
                    &indices,
                    &function_value.get_unique_reg_name(),
                );

                builder.build_store(index_pointer, value);
            }

            return BasicValueEnum::PointerValue(base_pointer);
        }

        Expression::ArrayMemberAccess { array, argument } => {
            let array_value = build_expression(
                array.as_ref(),
                context,
                builder,
                function_value,
                symbol_table,
                None,
            );

            if let BasicValueEnum::PointerValue(pointer) = array_value {

                let member_access_value = build_expression(
                    argument.as_ref(),
                    context,
                    builder,
                    function_value,
                    symbol_table,
                    None,
                );

                if let BasicValueEnum::FloatValue(float_value) = member_access_value {
                    let converted_int_value = builder.build_fp_to_si(float_value, context.i64_type(), function_value.get_unique_reg_name().as_str());
                    let indices = vec![context.i64_type().const_int(0, true), converted_int_value];

                    let array_type = pointer.get_type().into_array_type().unwrap();
                    let index_pointer = builder.build_gep_2(array_type, &pointer , &indices, function_value.get_unique_reg_name().as_str());
                    
                    let element_type = array_type.get_element_type();
                    let loaded_value = builder.build_load(index_pointer, element_type, name);
                    return loaded_value;

                } else {
                    panic!("Expected building expression in field 'argument' to give BasicValueEnum::FloatValue but got {:?}", member_access_value);
                }
            } else {
                panic!("Expected building expression in field 'array' to give BasicValueEnum::PointerValue but got {:?}", array_value);
            }
        }
    }
}

fn convert_data_type_to_array_type<'a>(
    data_type: &DataType,
    context: &'a Context,
    size: u32,
) -> Result<ArrayType<'a>, String> {
    let array_type = match data_type {
        DataType::Boolean => context.i1_type().array_type(size),
        DataType::Float => context.f64_type().array_type(size),

        _ => {
            return Err(format!(
                "Not possible to create array type for this dataType"
            ))
        }
    };

    return Ok(array_type);
}
