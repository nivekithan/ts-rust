use std::collections::HashMap;

use ast::{declaration::VariableAssignmentOperator, expression::Expression};
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    values::{enums::BasicValueEnum, fn_value::FunctionValue, ptr_value::PointerValue},
};

use crate::{build_assignment::build_assignment, build_expression::build_expression};

pub(crate) fn consume_array_member_assignments<'a>(
    ident_name: &str,
    member_access_exp: &Expression,
    operator: &VariableAssignmentOperator,
    exp: &Expression,
    context: &'a Context,
    builder: &'a Builder,
    function_value: &mut FunctionValue,
    symbol_table: &mut HashMap<String, PointerValue<'a>>,
    module: &'a Module,
) {
    let var_ptr = symbol_table.get(ident_name).unwrap().clone();
    let array_data_type = var_ptr.get_type().into_array_type().unwrap();

    let member_access_value = build_expression(
        member_access_exp,
        context,
        builder,
        function_value,
        symbol_table,
        module,
        None,
    );

    if let BasicValueEnum::FloatValue(member_access_value) = member_access_value {
        let converted_member_access_value = builder.build_fp_to_si(
            member_access_value,
            context.i64_type(),
            function_value.get_unique_reg_name().as_str(),
        );

        let indices = vec![
            context.i64_type().const_int(0, true),
            converted_member_access_value,
        ];

        let member_var_ptr = builder.build_gep_2(
            array_data_type,
            &var_ptr,
            &indices,
            function_value.get_unique_reg_name().as_str(),
        );
        let value = build_expression(
            exp,
            context,
            builder,
            function_value,
            symbol_table,
            module,
            None,
        );

        build_assignment(
            &member_var_ptr,
            &value,
            operator,
            context,
            builder,
            function_value,
            symbol_table,
        );
    } else {
        unreachable!();
    }
}
