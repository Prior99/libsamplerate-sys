extern crate bindgen;

use std::env;
use std::fs::{self, File};
use std::path::PathBuf;
use std::process::Command;

fn run(cmd: &mut Command) {
    match cmd.status() {
        Ok(status) => assert!(status.success()),
        Err(e) => panic!("Unable to execute {:?}! {}", cmd, e),
    }
}

fn build_libsamplerate() {
    let src = PathBuf::from(&env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("libsamplerate");
    let dst = PathBuf::from(&env::var_os("OUT_DIR").unwrap());
    match File::open(src.join("configure")) {
        Ok(_)  => {},
        Err(_) => {
            run(Command::new("./autogen.sh").current_dir(&src));
        },
    }
    run(Command::new("./configure").current_dir(&src));
    run(Command::new("make").current_dir(&src));
    let shlib = src.join("src/.libs");
    let _ = fs::copy(&shlib.join("libsamplerate.a"), &dst.join("libsamplerate.a"));
    println!("cargo:rustc-flags=-l static=samplerate");
    println!("cargo:rustc-flags=-L {}", dst.display());
}

fn main() {
    build_libsamplerate();
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
