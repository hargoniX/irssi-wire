use std::ffi::CString;
use std::os::raw::c_int;

mod irssi;
pub mod net;

use irssi::bindings::*;

#[no_mangle]
pub extern "C" fn wire_core_abicheck(version: *mut c_int) {
    unsafe {
        *version = 24;
    }
}

#[no_mangle]
pub extern "C" fn wire_core_init() {
    println!("HEY");
    unsafe {
        module_register_full(
            CString::new("wire").unwrap().as_ptr(),
            CString::new("core").unwrap().as_ptr(),
            CString::new("wire/core").unwrap().as_ptr(),
        );
    }
}

#[no_mangle]
pub extern "C" fn wire_core_deinit() {
    println!("BYE");
}
