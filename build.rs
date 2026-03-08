extern crate fs_extra;

use std::{env, path::PathBuf};

fn main() {
    build_and_link();
    add_bindings();
}

fn build_and_link() {
    let dst = cmake::Config::new("capstone")
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=capstone");

    let old_basedir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("capstone");
    let out_dir = env::var("OUT_DIR").map(PathBuf::from).unwrap();
    let basedir = out_dir.join("capstone");
    if !basedir.exists() {
        let mut opt = fs_extra::dir::CopyOptions::new();
        opt.overwrite = true;
        opt.copy_inside = true;
        fs_extra::dir::copy(old_basedir, &basedir, &opt).unwrap();
    }
    let include_dir = basedir.join("include");
    env::set_var("CAPSTONE_INCLUDE_DIR", &include_dir);
}

fn add_bindings() {
    let include_dir = env::var("CAPSTONE_INCLUDE_DIR").unwrap();
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .derive_debug(true)
        .impl_debug(true)
        .constified_enum("cs_mode")
        .clang_arg(format!("-I{}", include_dir))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
