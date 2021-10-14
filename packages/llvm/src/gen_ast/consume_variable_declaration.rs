use ast::expression::Expression;

use std::collections::HashMap;

use ast::data_type::DataType;
use inkwell::{
    builder::Builder,
    context::Context,
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
                pointer
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

        _ => todo!(),
    };

    symbol_table.insert(ident_name.to_owned(), pointer);
}
