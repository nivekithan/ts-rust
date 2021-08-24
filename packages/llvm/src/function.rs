use std::convert::TryInto;

use llvm_sys::{
    core::{LLVMAppendExistingBasicBlock, LLVMFunctionType},
    prelude::*,
};

use crate::{basic_block::BasicBlock, types::LlvmType, utils::is_valid_function_param_arguments};

pub struct LlvmFunction {
    llvm_ref: LLVMValueRef,
    context_ref: LLVMContextRef,
}

impl LlvmFunction {
    pub(crate) fn new(context_ref: LLVMContextRef, llvm_ref: LLVMValueRef) -> LlvmFunction {
        return LlvmFunction {
            llvm_ref,
            context_ref,
        };
    }

    pub(crate) fn get_ref(&self) -> LLVMValueRef {
        return self.llvm_ref;
    }

    pub(crate) fn get_context_ref(&self) -> LLVMContextRef {
        return self.context_ref;
    }

    pub fn append_existing_basic_block(&self, basic_block: &BasicBlock) {
        unsafe {
            LLVMAppendExistingBasicBlock(self.get_ref(), basic_block.get_ref());
        }
    }

    pub fn create_and_append_basic_block(&self, name: &str) -> BasicBlock {
        let basic_block = BasicBlock::from_context_ref(self.get_context_ref(), name);
        self.append_existing_basic_block(&basic_block);
        return basic_block;
    }
}

#[derive(Debug)]
pub struct LlvmFunctionType {
    llvm_ref: LLVMTypeRef,
}

impl LlvmFunctionType {
    pub fn new(
        param_types: &Vec<LlvmType>,
        return_type: &LlvmType,
    ) -> Result<LlvmFunctionType, String> {
        if !is_valid_function_param_arguments(&param_types) {
            return Err(format!(
                "Passed param_types contains one or more invalid function argument types {:?}",
                param_types
            ));
        };

        let mut c_param_types: Vec<LLVMTypeRef> = param_types
            .iter()
            .map(|types| {
                return types.get_ref();
            })
            .collect();

        let c_param_types = c_param_types.as_mut_ptr();

        unsafe {
            let llvm_ref = LLVMFunctionType(
                return_type.get_ref(),
                c_param_types,
                param_types.len().try_into().unwrap(),
                0,
            );

            return Ok(LlvmFunctionType { llvm_ref });
        }
    }

    pub(crate) fn get_ref(&self) -> LLVMTypeRef {
        return self.llvm_ref;
    }
}
