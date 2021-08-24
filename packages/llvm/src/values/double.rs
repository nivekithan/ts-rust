use llvm_sys::{
    core::{LLVMConstReal, LLVMDoubleTypeInContext},
    prelude::*,
};

use crate::context::Context;

pub struct LlvmDouble {
    llvm_ref: LLVMValueRef,
}

impl LlvmDouble {
    pub fn new_with_type(llvm_type: &LlvmDoubleType, num: f64) -> LlvmDouble {
        return LlvmDouble::new_from_type_ref(llvm_type.get_ref(), num);
    }

    pub fn new(context: &Context, num: f64) -> LlvmDouble {
        let double_type = LlvmDoubleType::new(context);
        return LlvmDouble::new_with_type(&double_type, num);
    }

    pub(crate) fn new_from_type_ref(llvm_type_ref: LLVMTypeRef, num: f64) -> LlvmDouble {
        unsafe {
            let llvm_ref = LLVMConstReal(llvm_type_ref, num);
            return LlvmDouble::new_from_value_ref(llvm_ref);
        }
    }

    pub(crate) fn new_from_value_ref(llvm_value_ref: LLVMValueRef) -> LlvmDouble {
        return LlvmDouble {
            llvm_ref: llvm_value_ref,
        };
    }

    pub(crate) fn get_ref(&self) -> LLVMValueRef {
        return self.llvm_ref;
    }
}

#[derive(Debug)]
pub struct LlvmDoubleType {
    llvm_ref: LLVMTypeRef,
}

impl LlvmDoubleType {
    pub fn new(context_ref: &Context) -> LlvmDoubleType {
        return LlvmDoubleType::new_from_ref(context_ref.get_ref());
    }

    pub(crate) fn new_from_ref(context_ref: LLVMContextRef) -> LlvmDoubleType {
        unsafe {
            let llvm_ref = LLVMDoubleTypeInContext(context_ref);
            return LlvmDoubleType { llvm_ref };
        }
    }

    pub(crate) fn get_ref(&self) -> LLVMTypeRef {
        return self.llvm_ref;
    }
}
