use llvm_sys::{
    core::{LLVMConstInt, LLVMGetIntTypeWidth},
    prelude::LLVMTypeRef,
};

use crate::values::int_value::IntValue;

use super::{
    array_type::ArrayType,
    enums::BasicTypeEnum,
    fn_type::FunctionType,
    traits::{AsTypeRef, BasicTypeTrait, IntMathTypeTrait},
    Type,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IntType<'a> {
    int_type: Type<'a>,
}

impl<'a> IntType<'a> {
    pub(crate) unsafe fn new(int_type: LLVMTypeRef) -> Self {
        assert!(!int_type.is_null());

        return IntType {
            int_type: Type::new(int_type),
        };
    }

    pub fn const_int(self, value: u64, sign_extended: bool) -> IntValue<'a> {
        unsafe {
            return IntValue::new(LLVMConstInt(
                self.as_type_ref(),
                value,
                sign_extended as i32,
            ));
        }
    }

    pub fn fn_type(
        self,
        param_types: &[BasicTypeEnum<'a>],
        variadic_arg: bool,
    ) -> FunctionType<'a> {
        return self.int_type.fn_type(param_types, variadic_arg);
    }

    pub fn array_type(self, size: u32) -> ArrayType<'a> {
        return self.int_type.array_type(size);
    }

    pub fn get_bit_width(self) -> u32 {
        unsafe {
            return LLVMGetIntTypeWidth(self.as_type_ref());
        }
    }
}

impl<'a> AsTypeRef for IntType<'a> {
    fn as_type_ref(&self) -> LLVMTypeRef {
        return self.int_type.as_type_ref();
    }
}

impl<'a> BasicTypeTrait<'a> for IntType<'a> {}

impl<'a> IntMathTypeTrait<'a> for IntType<'a> {
    type ValueType = IntValue<'a>;
}
