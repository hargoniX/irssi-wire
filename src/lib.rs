use std::os::raw::{c_char, c_int};
use std::ffi::CString;
mod bindings;

use bindings::*;

#[no_mangle]
pub extern fn wire_core_abicheck(version: *mut c_int) {
    unsafe {
        *version = 23;
    }
}

#[no_mangle]
pub extern fn wire_core_init() {
    println!("HEY");
    unsafe {
        module_register_full(CString::new("wire").unwrap().as_ptr(), CString::new("core").unwrap().as_ptr(), CString::new("wire/core").unwrap().as_ptr());
    }
}

#[no_mangle]
pub extern fn wire_core_deinit() {
    println!("BYE");
}

