use llvm_sys::{
    core::{LLVMCountParams, LLVMGetParam, LLVMIsAFunction},
    prelude::LLVMValueRef,
};

use super::{enums::BasicValueEnum, traits::AsValueRef, Value};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]

pub struct FunctionValue<'a> {
    pub(crate) fn_value: Value<'a>,

    reg_counter: usize,
    block_counter: usize,
}

impl<'a> FunctionValue<'a> {
    pub(crate) unsafe fn new(value: LLVMValueRef) -> Self {
        assert!(!value.is_null());

        assert!(!LLVMIsAFunction(value).is_null());

        return FunctionValue {
            fn_value: Value::new(value),
            reg_counter: 0,
            block_counter: 0,
        };
    }

    pub fn get_unique_reg_name(&mut self) -> String {
        let cur_reg_count = self.reg_counter;
        let reg_name = format!("{}", cur_reg_count);
        self.reg_counter += 1;

        return reg_name;
    }

    pub fn get_unique_block_name(&mut self) -> String {
        let block_counter = self.block_counter;
        let block_name = format!("Block_{}", block_counter);
        self.block_counter += 1;

        return block_name;
    }

    pub fn set_reg_counter(&mut self, new_num: usize) {
        self.reg_counter = new_num;
    }

    pub fn count_params(&self) -> u32 {
        unsafe {
            return LLVMCountParams(self.as_value_ref());
        }
    }

    pub fn get_nth_param(&self, nth: u32) -> Option<BasicValueEnum<'a>> {
        let count = self.count_params();

        if nth + 1 > count {
            return None;
        }

        unsafe {
            return Some(BasicValueEnum::new(LLVMGetParam(self.as_value_ref(), nth)));
        }
    }
}

impl<'a> AsValueRef for FunctionValue<'a> {
    fn as_value_ref(&self) -> LLVMValueRef {
        return self.fn_value.as_value_ref();
    }
}
