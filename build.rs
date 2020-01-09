use std::path::PathBuf;
use std::env;
use std::io::prelude::*;
use std::fs::File;

use protoc_rust::Customize;

fn main() {
    if env::var("REBUILD_BINDGEN").is_ok() {
        // C bindings
        eprintln!("Generating C bindings to irssi");
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
        let bindings = bindgen::Builder::default()
            .header("headers.h")
            .clang_arg("-I/usr/include/glib-2.0/")
            .clang_arg(irssi_path)
            .clang_arg("-I/usr/lib/x86_64-linux-gnu/glib-2.0/include/")
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file("src/irssi/bindgen_output.rs")
            .expect("Couldn't write bindings!");
    }
    else {
        eprintln!("Skipping C binding generation");
    }

    // Protobuffer code generation
    if env::var("REBUILD_PROTOBUF").is_ok() {
        eprintln!("Generating protobuf code");
        protoc_rust::run(protoc_rust::Args {
            out_dir: "src/net/protos",
            input: &["wire_websocket.proto"],
            includes: &[],
            customize: Customize {
                ..Default::default()
            },
        }).expect("Couln't generate code from protobuffers for wire api");
    }
    else {
        eprintln!("Skipping protobuf generation");
    }
}
