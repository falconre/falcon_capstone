extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    #[cfg(not(feature = "static"))]
    println!("cargo:rustc-link-lib=capstone");

    #[cfg(feature = "static")]
    println!("cargo:rustc-link-lib=static=capstone");

    // https://github.com/aquynh/capstone/blob/71f5c64c43b9868ab08c3a9bad82ba3563e9436d/COMPILE.TXT#L100
    #[cfg(all(feature = "static", target_os = "linux"))]
    println!("cargo:rustc-link-search=native=/usr/lib");

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
