use std::ffi::{CString, NulError};

use crate::types::LlvmType;

pub(crate) fn is_valid_function_param_arguments(param_types: &Vec<LlvmType>) -> bool {
    for llvm_type in param_types {
        match llvm_type {
            &LlvmType::Double(_) => {
                continue;
            }
            &LlvmType::Function(_) => {
                return false;
            }
        }
    }

    return true;
}

pub(crate) fn convert_to_c_string(str: &str) -> Result<*const i8, NulError> {
    let c_string = CString::new(str)?;
    let ptr = c_string.as_ptr();

    return Ok(ptr);
}
