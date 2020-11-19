extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut config = cmake::Config::new("libsamplerate");
    config.build_target("samplerate");
    let mut path = config.build();
    if std::env::var("TARGET").unwrap().contains("msvc") {
        path = path.join("build").join(config.get_profile());
    } else {
        path = path.join("build");
    }
    println!("cargo:rustc-link-search=native={}", path.display());
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
