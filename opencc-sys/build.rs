use bindgen::Builder;
use std::env;
use std::path::PathBuf;

fn main() {
    let mut cfg = &mut cmake::Config::new("OpenCC");

    let build_type = "Release";

    cfg = cfg
        .define("CMAKE_BUILD_TYPE", build_type)
        .define("BUILD_DOCUMENTATION", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("ENABLE_GTEST", "OFF")
        .define("ENABLE_BENCHMARK", "OFF")
        .define("ENABLE_DARTS", "OFF")
        .define("BUILD_PYTHON", "OFF")
        .define("BUILD_TESTING", "OFF")
        .define("USE_SYSTEM_DARTS", "OFF")
        .define("USE_SYSTEM_GOOGLE_BENCHMARK", "OFF")
        .define("USE_SYSTEM_GTEST", "OFF")
        .define("USE_SYSTEM_MARISA", "OFF")
        .define("USE_SYSTEM_PYBIND11", "OFF")
        .define("USE_SYSTEM_RAPIDJSON", "OFF")
        .define("USE_SYSTEM_TCLAP", "OFF")
        .profile(build_type)
        .very_verbose(true);

    let mut marisa = cfg.build_target("marisa").build();
    let mut opencc = cfg.build_target("libopencc").build();
    cfg.build_target("Dictionaries").build();

    marisa = marisa.join("build").join("deps").join("marisa-0.2.6");
    opencc = opencc.join("build").join("src");
    if cfg!(host_family = "windows") {
        marisa = marisa.join("Release");
        opencc = opencc.join("Release");
    }

    println!("cargo:rustc-link-search=native={}", marisa.display());
    println!("cargo:rustc-link-search=native={}", opencc.display());
    println!("cargo:rustc-link-lib=static=marisa");
    println!("cargo:rustc-link-lib=static=opencc");

    println!(
        "cargo:rustc-link-lib={}",
        if cfg!(target_os = "macos") {
            "c++".to_string()
        } else {
            "stdc++".to_string()
        }
    );

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = Builder::default()
        .clang_arg("-IOpenCC/src")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
