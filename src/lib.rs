use std::os::raw::c_int;
use std::ffi::CString;
use std::thread;
use std::time::Duration;

mod irssi;
mod net;
mod storage;

use irssi::bindings::*;

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
    thread::spawn(|| {
        let mut i = 0;
        loop {
            thread::sleep(Duration::from_secs(10));
            println!("{} iteration", i);
            i += 1;
        }
    });
}

#[no_mangle]
pub extern fn wire_core_deinit() {
    println!("BYE");
}

