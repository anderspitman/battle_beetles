extern crate protoc_rust;

use std::process::Command;

fn main() {
    
    // generate rust protobuf files
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/gen",
        input: &["protos/messages.proto"],
        includes: &["protos"],
    }).expect("protoc");

    // generate js protobuf files
    let status = Command::new("protoc").args(&[
        "--proto_path=protos",
        "--js_out=import_style=commonjs,binary:ui/gen",
        "protos/messages.proto"]).status();
    match status {
        Ok(s) => {
            if !s.success() {
                println!("protoc failed: {}", s);
            }
        },
        Err(e) => {
            println!("protoc command failed: {}", e);
        }
    };

    // bundle js
    let status = Command::new("npm").args(&[
        "run", "--prefix", "ui", "build",
    ]).status();
    match status {
        Ok(s) => {
            if !s.success() {
                println!("npm failed: {}", s);
            }
        },
        Err(e) => {
            println!("npm command failed: {}", e);
        }
    };

    println!("cargo:rerun-if-changed=protos/messages.proto");
    println!("cargo:rerun-if-changed=ui/index.html");
    println!("cargo:rerun-if-changed=ui/index.js");
    println!("cargo:rerun-if-changed=ui/message_service.js");
}
