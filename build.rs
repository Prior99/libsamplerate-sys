extern crate cmake;

fn main() {
    let mut config = cmake::Config::new("libsamplerate");
    config
        .define("LIBSAMPLERATE_TESTS", "OFF")
        .define("LIBSAMPLERATE_EXAMPLES", "OFF")
        .define("LIBSAMPLERATE_INSTALL", "OFF")
        .build_target("samplerate");

    let target = std::env::var("TARGET").unwrap();
    if target.contains("darwin") {
        let cmake_osx_arch = if target.contains("aarch64") {
            "arm64"
        } else {
            "x86_64"
        };
        config.define("CMAKE_OSX_ARCHITECTURES", cmake_osx_arch);
    }

    let mut path = config.build();

    if target.contains("msvc") {
        path = path.join("build").join(config.get_profile());
    } else {
        path = path.join("build");
    }

    println!("cargo:rustc-link-search=native={}", path.display());
    println!("cargo:rustc-link-lib=static=samplerate");
}
