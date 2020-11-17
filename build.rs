extern crate cmake;

fn main() {
    let libsr = cmake::build("libsamplerate");
    println!("cargo:rustc-link-search=native={}", libsr.join("lib").display());
    println!("cargo:rustc-link-lib=static=samplerate");
}
