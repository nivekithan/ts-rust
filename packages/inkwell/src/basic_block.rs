use std::marker::PhantomData;

use llvm_sys::prelude::LLVMBasicBlockRef;

#[derive(Debug, Clone, PartialEq)]
pub struct BasicBlock<'a> {
    pub(crate) basic_block: LLVMBasicBlockRef,
    _marker: PhantomData<&'a ()>,
}

impl<'a> BasicBlock<'a> {
    pub(crate) unsafe fn new(basic_block: LLVMBasicBlockRef) -> Self {
        assert!(!basic_block.is_null());

        return BasicBlock {
            basic_block,
            _marker: PhantomData,
        };
    }
}
