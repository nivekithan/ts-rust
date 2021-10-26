use ast::expression::Expression;

use std::collections::HashMap;

use ast::data_type::DataType;
use inkwell::{
    builder::Builder,
    context::Context,
    types::traits::BasicTypeTrait,
    values::{enums::BasicValueEnum, fn_value::FunctionValue, ptr_value::PointerValue},
};

use crate::build_expression::build_expression;

pub(crate) fn consume_variable_declaration<'a>(
    ident_name: &String,
    exp: &Expression,
    context: &'a Context,
    builder: &'a Builder,
    function_value: &mut FunctionValue,
    symbol_table: &mut HashMap<String, PointerValue<'a>>,
) {
    let data_type = exp.get_data_type();

    let pointer = match data_type {
        DataType::Float => {
            let pointer = builder.build_alloca(context.f64_type(), ident_name.as_str());
            let value_of_exp =
                build_expression(exp, context, builder, function_value, symbol_table, None);
            builder.build_store(pointer.clone(), value_of_exp);

            pointer
        }

        DataType::Boolean => {
            let pointer = builder.build_alloca(context.i1_type(), ident_name.as_str());
            let value_of_exp =
                build_expression(exp, context, builder, function_value, symbol_table, None);
            builder.build_store(pointer.clone(), value_of_exp);

            pointer
        }

        DataType::String => {
            let value = build_expression(
                exp,
                context,
                builder,
                function_value,
                symbol_table,
                Some(ident_name.to_string()),
            );
            if let BasicValueEnum::PointerValue(pointer) = value {
                if let Expression::StringLiteralExp { value: _ } = exp {
                    pointer
                } else {
                    let string_type = pointer.get_type().into_array_type().unwrap();
                    let size = string_type.get_length();

                    let new_pointer = builder.build_alloca(string_type, &ident_name);

                    for x in 0..size {
                        let indices = vec![
                            context.i64_type().const_int(0, true),
                            context.i64_type().const_int(x.into(), true),
                        ];

                        let original_index_pointer = builder.build_gep_2(
                            string_type,
                            &pointer,
                            &indices,
                            &function_value.get_unique_reg_name(),
                        );

                        let new_index_pointer = builder.build_gep_2(
                            string_type,
                            &new_pointer,
                            &indices,
                            &function_value.get_unique_reg_name(),
                        );

                        let char_value = builder.build_load(
                            original_index_pointer,
                            context.i8_type().as_basic_type_enum(),
                            &function_value.get_unique_reg_name(),
                        );

                        builder.build_store(new_index_pointer, char_value);
                    }

                    new_pointer
                }
            } else {
                todo!();
            }
        }

        DataType::ArrayType { base_type: _ } => {
            let value = build_expression(
                exp,
                context,
                builder,
                function_value,
                symbol_table,
                Some(ident_name.to_string()),
            );

            if let BasicValueEnum::PointerValue(pointer) = value {
                pointer
            } else {
                todo!();
            }
        }

        DataType::ObjectType { entries: _ } => {
            let value = build_expression(
                exp,
                context,
                builder,
                function_value,
                symbol_table,
                Some(ident_name.to_string()),
            );

            if let BasicValueEnum::PointerValue(pointer) = value {
                pointer
            } else {
                panic!("Expected function build_expression for DataType::ObjectType to return BasicValueEnum::PointerValue")
            }
        }

        DataType::Unknown => unreachable!(),
    };

    symbol_table.insert(ident_name.to_owned(), pointer);
}
