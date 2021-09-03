use llvm_sys::prelude::LLVMValueRef;

use super::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstructionValue<'a> {
    instruction_value: Value<'a>,
}

impl<'a> InstructionValue<'a> {
    pub(crate) unsafe fn new(instruction_value: LLVMValueRef) -> Self {
        assert!(!instruction_value.is_null());

        return InstructionValue {
            instruction_value: Value::new(instruction_value),
        };
    }
}
