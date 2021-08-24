use llvm_sys::{
    core::{LLVMBuildAnd, LLVMCreateBasicBlockInContext},
    prelude::*,
};

use crate::{
    builder::Builder, context::Context, function::LlvmFunction, utils::convert_to_c_string,
    values::double::LlvmDouble,
};

pub struct BasicBlock {
    llvm_ref: LLVMBasicBlockRef,
    builder: Builder,
}

impl BasicBlock {
    pub fn new(context: &Context, name: &str) -> BasicBlock {
        let context_ref = context.get_ref();
        return BasicBlock::from_context_ref(context_ref, name);
    }

    pub(crate) fn from_context_ref(context_ref: LLVMContextRef, name: &str) -> BasicBlock {
        let c_name = convert_to_c_string(name).unwrap();

        unsafe {
            let llvm_ref = LLVMCreateBasicBlockInContext(context_ref, c_name);
            let builder = Builder::from_context_ref(context_ref);
            let basic_block = BasicBlock { llvm_ref, builder };
            basic_block.sync_builder();
            return basic_block;
        }
    }

    pub(crate) fn get_ref(&self) -> LLVMBasicBlockRef {
        return self.llvm_ref;
    }

    // pub(crate) fn get_value_ref(&self) -> LLVMValueRef {
    //     unsafe {
    //         let value_ref = LLVMBasicBlockAsValue(self.get_ref());
    //         return value_ref;
    //     }
    // }

    pub(crate) fn get_builder_ref(&self) -> LLVMBuilderRef {
        return self.builder.get_ref();
    }

    pub fn sync_builder(&self) {
        self.builder.position_at_end_of_basic_block(self);
    }

    pub fn append_to_function(&self, value: &LlvmFunction) {
        value.append_existing_basic_block(self);
    }

    pub(crate) fn add_double_from_ref(
        &self,
        lhs: LLVMValueRef,
        rhs: LLVMValueRef,
        name: &str,
    ) -> LlvmDouble {
        let builder_ref = self.get_builder_ref();
        let c_name = convert_to_c_string(name).unwrap();

        unsafe {
            let value_ref = LLVMBuildAnd(builder_ref, lhs, rhs, c_name);
            let double_value = LlvmDouble::new_from_value_ref(value_ref);
            return double_value;
        }
    }

    pub fn add_double(&self, lhs: &LlvmDouble, rhs: &LlvmDouble, name: &str) -> LlvmDouble {
        let lhs_ref = lhs.get_ref();
        let rhs_ref = rhs.get_ref();

        return self.add_double_from_ref(lhs_ref, rhs_ref, name);
    }
}
