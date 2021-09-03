use std::ffi::CString;

pub mod llvm_string;

pub(crate) fn to_c_str(name: &str) -> CString {
    let c_name = CString::new(name).unwrap();
    return c_name;
}
