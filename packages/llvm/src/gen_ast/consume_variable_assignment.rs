use ast::{declaration::VariableAssignmentOperator, expression::Expression};
use inkwell::{
    builder::Builder, context::Context, module::Module, values::fn_value::FunctionValue,
};

use crate::{
    build_assignment::build_assignment, build_expression::build_expression,
    symbol_table::SymbolTable,
};

pub(crate) fn consume_variable_assignment<'a>(
    ident_name: &String,
    exp: &Expression,
    operator: &VariableAssignmentOperator,
    context: &'a Context,
    builder: &'a Builder,
    function_value: &mut FunctionValue,
    symbol_table: &mut SymbolTable<'a>,
    module: &'a Module,
) {
    let var_ptr = symbol_table.get(ident_name).unwrap().clone();
    let value = build_expression(
        exp,
        context,
        builder,
        function_value,
        symbol_table,
        module,
        None,
    )
    .unwrap();

    build_assignment(
        &var_ptr,
        &value,
        operator,
        context,
        builder,
        function_value,
        symbol_table,
    );
}
