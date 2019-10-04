extern crate bindgen;

use std::path::PathBuf;
use std::env;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let server_rec = "
typedef struct _WireServerRec {
    int x;
} WireServerRec;

#define STRUCT_SERVER_REC WireServerRec".to_string();
    let mut recs = File::create("recs.h").unwrap(); 
    recs.write_all(server_rec.as_bytes()).unwrap();
    

    // Generate bindings for headers.h
    let mut irssi_path = "-I".to_string();
    irssi_path.push_str(env::var("IRSSI_INCLUDE").unwrap().as_str());
    println!("I AM IRSSI PATH: {}", irssi_path);
    let bindings = bindgen::Builder::default()
        .header("headers.h")
        .clang_arg("-I/usr/include/glib-2.0/")
        .clang_arg(irssi_path)
        .clang_arg("-I/usr/lib/x86_64-linux-gnu/glib-2.0/include/")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
