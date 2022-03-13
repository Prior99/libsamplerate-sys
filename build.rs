extern crate cmake;

fn main() {
    let manifest_path = env!("CARGO_MANIFEST_DIR");
    let mut config = cmake::Config::new("libsamplerate");
    config
        .define("LIBSAMPLERATE_TESTS", "OFF")
        .define("LIBSAMPLERATE_EXAMPLES", "OFF")
        .define("LIBSAMPLERATE_INSTALL", "OFF");

    if std::env::var("TARGET").unwrap().contains("-ios") {
        config
            .define("CMAKE_TOOLCHAIN_FILE", format!("{}/ios-cmake/ios.toolchain.cmake", manifest_path))
            .define("PLATFORM", "OS64")
            .define("IOS_DEPLOYMENT_TARGET", "11.0")
            .define("CMAKE_OSX_SYSROOT", "/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk")
            .generator("Xcode");
    }

    config
        .build_target("samplerate");

    let mut path = config
        .very_verbose(true)
        .build();

    if std::env::var("TARGET").unwrap().contains("msvc") {
        path = path.join("build").join(config.get_profile());
    } else if std::env::var("TARGET").unwrap().contains("-ios") {
        path = path.join("build").join("Release-iphoneos");
    } else {
        path = path.join("build");
    }

    println!("cargo:rustc-link-search=native={}", path.display());
    println!("cargo:rustc-link-lib=static=samplerate");
}
