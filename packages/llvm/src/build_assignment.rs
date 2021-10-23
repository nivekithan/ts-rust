use std::collections::HashMap;

use ast::declaration::VariableAssignmentOperator;
use inkwell::{
    builder::Builder,
    context::Context,
    types::traits::BasicTypeTrait,
    values::{enums::BasicValueEnum, fn_value::FunctionValue, ptr_value::PointerValue},
};

/*
 * When given a pointerValue, assignmentOperator, BasicValue
 * This function will store BasicValue in the place the pointerValue points to
 * based on assignmentOperator
 *
 *
 *  */
pub(crate) fn build_assignment<'a>(
    var_ptr: &PointerValue,
    value: &BasicValueEnum,
    operator: &VariableAssignmentOperator,
    context: &'a Context,
    builder: &'a Builder,
    function_value: &mut FunctionValue,
    _symbol_table: &mut HashMap<String, PointerValue<'a>>,
) {
    let var_ptr = var_ptr.clone();
    let value = value.clone();

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
