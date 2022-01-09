use crate::values::float_value::FloatValue;

use super::{
    array_type::ArrayType,
    enums::BasicTypeEnum,
    fn_type::FunctionType,
    traits::{AsTypeRef, BasicTypeTrait, FloatMathTypeTrait},
    Type,
};
use llvm_sys::{core::LLVMConstReal, prelude::LLVMTypeRef};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]

pub struct FloatType<'a> {
    float_type: Type<'a>,
}

impl<'a> FloatType<'a> {
    pub(crate) unsafe fn new(float_type: LLVMTypeRef) -> Self {
        assert!(!float_type.is_null());

        return FloatType {
            float_type: Type::new(float_type),
        };
    }

    pub fn fn_type(self, param_types: &[BasicTypeEnum], variadic_arg: bool) -> FunctionType<'a> {
        return self.float_type.fn_type(param_types, variadic_arg);
    }

    pub fn array_type(self, size: u32) -> ArrayType<'a> {
        return self.float_type.array_type(size);
    }

    pub fn const_float(self, value: f64) -> FloatValue<'a> {
        unsafe {
            return FloatValue::new(LLVMConstReal(self.float_type.ty, value));
        }
    }
}

impl<'a> AsTypeRef for FloatType<'a> {
    fn as_type_ref(&self) -> LLVMTypeRef {
        return self.float_type.as_type_ref();
    }
}

impl<'a> BasicTypeTrait<'a> for FloatType<'a> {}

impl<'a> FloatMathTypeTrait<'a> for FloatType<'a> {
    type ValueType = FloatValue<'a>;
}
