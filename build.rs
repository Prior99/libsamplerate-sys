extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

fn main() {
    let libsr = cmake::Config::new("libsamplerate")
        .build_target("samplerate")
        .build();
    let path = libsr.join("build");

    println!("cargo:rustc-link-search=native={}", path.display());
    // Debug/Release/... paths for MSVC:
    println!("cargo:rustc-link-search=native={}", path.join("Debug").display());
    println!("cargo:rustc-link-search=native={}", path.join("Release").display());
    println!("cargo:rustc-link-search=native={}", path.join("RelWithDebInfo").display());
    println!("cargo:rustc-link-search=native={}", path.join("MinSizeRel").display());
    println!("cargo:rustc-link-lib=static=samplerate");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
