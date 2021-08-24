use llvm_sys::{
    core::{LLVMCreateBuilderInContext, LLVMDisposeBuilder, LLVMPositionBuilderAtEnd},
    prelude::*,
};

use crate::{basic_block::BasicBlock, context::Context};

pub struct Builder {
    llvm_ref: LLVMBuilderRef,
}

impl Builder {
    pub fn new(context: &Context) -> Builder {
        let context_ref = context.get_ref();
        return Builder::from_context_ref(context_ref);
    }

    pub(crate) fn from_context_ref(context_ref: LLVMContextRef) -> Builder {
        unsafe {
            let llvm_ref = LLVMCreateBuilderInContext(context_ref);
            return Builder { llvm_ref };
        }
    }

    pub fn position_at_end_of_basic_block(&self, basic_block: &BasicBlock) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.get_ref(), basic_block.get_ref());
        }
    }

    pub(crate) fn get_ref(&self) -> LLVMBuilderRef {
        return self.llvm_ref;
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.get_ref());
        }
    }
}
