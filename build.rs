extern crate protoc_rust;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/gen",
        input: &["protos/messages.proto"],
        includes: &["protos"],
    }).expect("protoc");

    println!("cargo:rerun-if-changed=protos");
}
