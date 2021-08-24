use llvm_sys::prelude::*;

use crate::{function::LlvmFunctionType, values::double::LlvmDoubleType};

#[derive(Debug)]
pub enum LlvmType {
    Function(LlvmFunctionType),
    Double(LlvmDoubleType),
}

impl LlvmType {
    pub fn get_ref(&self) -> LLVMTypeRef {
        match self {
            LlvmType::Function(f_type) => return f_type.get_ref(),
            LlvmType::Double(d_type) => return d_type.get_ref(),
        }
    }
}
