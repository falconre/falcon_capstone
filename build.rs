extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=capstone");

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
        .derive_debug(true)
        .impl_debug(true)
        .constified_enum("cs_mode")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
