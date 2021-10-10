use std::collections::HashMap;

use ast::{declaration::VariableAssignmentOperator, expression::Expression};
use inkwell::{
    builder::Builder,
    context::Context,
    types::traits::BasicTypeTrait,
    values::{enums::BasicValueEnum, fn_value::FunctionValue, ptr_value::PointerValue},
};

use crate::build_expression::build_expression;

pub(crate) fn consume_variable_assignment<'a>(
    ident_name: &String,
    exp: &Expression,
    operator: &VariableAssignmentOperator,
    context: &'a Context,
    builder: &'a Builder,
    function_value: &mut FunctionValue,
    symbol_table: &mut HashMap<String, PointerValue<'a>>,
) {
    let var_ptr = symbol_table.get(ident_name).unwrap().clone();
    let value = build_expression(exp, context, builder, function_value, symbol_table, None);

    match operator {
        VariableAssignmentOperator::Assign => {
            builder.build_store(var_ptr, value);
        }

        VariableAssignmentOperator::PlusAssign => {
            let load_value = builder.build_load(
                var_ptr,
                context.f64_type().as_basic_type_enum(),
                function_value.get_unique_reg_name().as_str(),
            );

            if let BasicValueEnum::FloatValue(lhs) = load_value {
                if let BasicValueEnum::FloatValue(rhs) = value {
                    let result_value = builder.build_float_add(
                        lhs,
                        rhs,
                        function_value.get_unique_reg_name().as_str(),
                    );
                    builder.build_store(var_ptr, result_value);
                }
            }
        }

        VariableAssignmentOperator::MinusAssign => {
            let load_value = builder.build_load(
                var_ptr,
                context.f64_type().as_basic_type_enum(),
                function_value.get_unique_reg_name().as_str(),
            );

            if let BasicValueEnum::FloatValue(lhs) = load_value {
                if let BasicValueEnum::FloatValue(rhs) = value {
                    let result_value = builder.build_float_sub(
                        lhs,
                        rhs,
                        function_value.get_unique_reg_name().as_str(),
                    );
                    builder.build_store(var_ptr, result_value);
                }
            }
        }

        VariableAssignmentOperator::StarAssign => {
            let load_value = builder.build_load(
                var_ptr,
                context.f64_type().as_basic_type_enum(),
                function_value.get_unique_reg_name().as_str(),
            );

            if let BasicValueEnum::FloatValue(lhs) = load_value {
                if let BasicValueEnum::FloatValue(rhs) = value {
                    let result_value = builder.build_float_mul(
                        lhs,
                        rhs,
                        function_value.get_unique_reg_name().as_str(),
                    );
                    builder.build_store(var_ptr, result_value);
                }
            }
        }

        VariableAssignmentOperator::SlashAssign => {
            let load_value = builder.build_load(
                var_ptr,
                context.f64_type().as_basic_type_enum(),
                function_value.get_unique_reg_name().as_str(),
            );

            if let BasicValueEnum::FloatValue(lhs) = load_value {
                if let BasicValueEnum::FloatValue(rhs) = value {
                    let result_value = builder.build_float_div(
                        lhs,
                        rhs,
                        function_value.get_unique_reg_name().as_str(),
                    );
                    builder.build_store(var_ptr, result_value);
                }
            }
        } // _ => todo!(),
    };
}
