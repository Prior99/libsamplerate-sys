extern crate cmake;

fn main() {
    let mut config = cmake::Config::new("libsamplerate");
    config
        .define("LIBSAMPLERATE_TESTS", "OFF")
        .define("LIBSAMPLERATE_EXAMPLES", "OFF")
        .define("LIBSAMPLERATE_INSTALL", "OFF")
        .define("CMAKE_OSX_DEPLOYMENT_TARGET", "11.0")
        .define("CMAKE_SYSTEM_NAME", "iOS")
        .define("CMAKE_Swift_COMPILER_FORCED", "true")
        .very_verbose(true)
        .build_target("samplerate");

    let mut path = config
        .very_verbose(true)
        .build();

    if std::env::var("TARGET").unwrap().contains("msvc") {
        path = path.join("build").join(config.get_profile());
    } else {
        path = path.join("build");
    }
    println!("cargo:rustc-link-search=native={}", path.display());
    println!("cargo:rustc-link-lib=static=samplerate");
}
