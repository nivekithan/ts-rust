use llvm_sys::{
    core::LLVMGetTypeKind,
    prelude::{LLVMTypeRef, LLVMValueRef},
    LLVMTypeKind,
};

use super::{enums::BasicValueEnum, traits::AsValueRef, Value};

/* Represents the value that is returned by calling a function by any opeartion
 * like `invoke` or `call`
 *
 * It can only be either value supported by BasicValueEnum or Void
 *
 *  */

pub struct ReturnedValue<'a> {
    returned_value: Value<'a>,
}

impl<'a> ReturnedValue<'a> {
    pub unsafe fn new(value_ref: LLVMValueRef) -> Self {
        assert!(!value_ref.is_null());

        return ReturnedValue {
            returned_value: Value::new(value_ref),
        };
    }

    pub unsafe fn get_type(&self) -> LLVMTypeRef {
        return self.returned_value.get_type();
    }

    pub fn is_void(&self) -> bool {
        unsafe {
            let self_type = self.get_type();

            if LLVMTypeKind::LLVMVoidTypeKind == LLVMGetTypeKind(self_type) {
                return true;
            } else {
                return false;
            }
        }
    }

    pub fn to_basic_value_enum(&self) -> Result<BasicValueEnum<'a>, ()> {
        unsafe {
            if self.is_void() {
                return Err(());
            } else {
                return Ok(BasicValueEnum::new(self.as_value_ref()));
            }
        }
    }
}

impl<'a> AsValueRef for ReturnedValue<'a> {
    fn as_value_ref(&self) -> LLVMValueRef {
        return self.returned_value.as_value_ref();
    }
}
