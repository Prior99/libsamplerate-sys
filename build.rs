extern crate bindgen;
extern crate pathdiff;

use pathdiff::diff_paths;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn run(cmd: &mut Command) {
    match cmd.status() {
        Ok(status) => assert!(status.success()),
        Err(e) => panic!("Unable to execute {:?}! {}", cmd, e),
    }
}

fn build_libsamplerate_unix(src: &PathBuf, dst: &PathBuf) {
    run(Command::new("cmake").args(&["-DCMAKE_C_FLAGS=-fPIC", diff_paths(&src, &dst).unwrap().to_str().unwrap()]).current_dir(&dst));
    run(Command::new("make").args(&["samplerate"]).current_dir(&dst));
    let shlib = src.join("src/.libs");
    let _ = fs::copy(&shlib.join("libsamplerate.a"), &dst.join("libsamplerate.a"));
    println!("cargo:rustc-flags=-l static=samplerate");
    println!("cargo:rustc-flags=-L {}", dst.display());
    println!("cargo:rerun-if-changed={}", src.to_str().unwrap());
}

fn build_libsamplerate_windows(src: &PathBuf, dst: &PathBuf) {
    run(Command::new("cmake").args(&[
        "-DCMAKE_SYSTEM_NAME=Windows",
        "-DCMAKE_C_COMPILER=x86_64-w64-mingw32-gcc",
        "-DCMAKE_C_FLAGS=-fPIC",
        "-DBUILD_SHARED_LIBS=off",
        "-DWIN32=on",
        "-DSNDFILE_INCLUDE_DIR=/usr/include",
        "-DBUILD_EXAMPLES=OFF",
        diff_paths(&src, &dst).unwrap().to_str().unwrap(),
    ]).current_dir(&dst));
    run(Command::new("make").args(&["samplerate"]).current_dir(&dst));
    println!("cargo:rustc-flags=-l static=samplerate");
    println!("cargo:rustc-flags=-L {}", dst.display());
    println!("cargo:rerun-if-changed={}", src.to_str().unwrap());
}

fn main() {
    let src = PathBuf::from(&env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("libsamplerate");
    let dst = PathBuf::from(&env::var_os("OUT_DIR").unwrap());
    let _ = fs::create_dir(&dst);
    match env::var("CARGO_CFG_TARGET_OS").as_ref().map(|x| &**x) {
        Ok("windows") => build_libsamplerate_windows(&src, &dst),
        _ => build_libsamplerate_unix(&src, &dst),
    };
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
