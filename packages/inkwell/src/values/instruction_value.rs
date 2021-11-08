use llvm_sys::prelude::LLVMValueRef;

use super::{enums::BasicValueEnum, traits::AsValueRef, Value};

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

    pub fn try_as_basic_value(&self) -> Result<BasicValueEnum<'a>, String> {
        unsafe {
            if BasicValueEnum::is_basic_value_enum(self.as_value_ref()) {
                return Ok(BasicValueEnum::new(self.as_value_ref()));
            } else {
                return Err(format!(
                    "It is not possible to create BasicValueEnum from this instruction"
                ));
            }
        }
    }
}

impl<'a> AsValueRef for InstructionValue<'a> {
    fn as_value_ref(&self) -> LLVMValueRef {
        return self.instruction_value.as_value_ref();
    }
}
