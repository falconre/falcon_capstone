extern crate fs_extra;

use std::env;
use std::path::PathBuf;

fn main() {
    build::build_and_link();
    add_bindings();
}

pub fn cargo_rerun_if_env_changed(env_var: &str) {
    let target = std::env::var("TARGET").unwrap();
    println!("cargo:rerun-if-env-changed={}", env_var);
    println!("cargo:rerun-if-env-changed={}_{}", env_var, target);
    println!(
        "cargo:rerun-if-env-changed={}_{}",
        env_var,
        target.replace('-', "_")
    );
}

pub fn get_target_env_var(env_var: &str) -> Option<String> {
    let target = std::env::var("TARGET").unwrap();
    std::env::var(format!("{}_{}", env_var, target))
        .or_else(|_| std::env::var(format!("{}_{}", env_var, target.replace('-', "_"))))
        .or_else(|_| std::env::var(env_var))
        .ok()
}

pub fn is_enable(env_var: &str, default: bool) -> bool {
    match get_target_env_var(env_var).as_deref() {
        Some("0") => false,
        Some(_) => true,
        None => default,
    }
}

#[cfg(all(feature = "vendored"))]
mod build {
    use fs_extra::dir::{copy, CopyOptions};
    use std::path::PathBuf;

    pub fn build_and_link() {
        let dst = cmake::Config::new("capstone")
            .define("BUILD_SHARED_LIBS", "OFF")
            .build();

        println!("cargo:rustc-link-search=native={}/lib", dst.display());
        println!("cargo:rustc-link-lib=static=capstone");
        //        let dst = cmake::build("capstone");
        let old_basedir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("capstone");
        let out_dir = std::env::var("OUT_DIR").map(PathBuf::from).unwrap();
        let basedir = out_dir.join("capstone");
        if !basedir.exists() {
            let mut opt = CopyOptions::new();
            opt.overwrite = true;
            opt.copy_inside = true;
            copy(old_basedir, &basedir, &opt).unwrap();
        }
        let include_dir = basedir.join("include");
        std::env::set_var("CAPSTONE_INCLUDE_DIR", include_dir);
    }
}

#[cfg(not(feature = "vendored"))]
mod build {
    pub fn build_and_link() {
        let kind = if super::is_enable("LIBCAPSTONE_STATIC", false) {
            "static"
        } else {
            "dylib"
        };
        println!("cargo:rustc-link-lib={}=capstone", kind);
        super::cargo_rerun_if_env_changed("LIBCAPSTONE_STATIC");
        super::cargo_rerun_if_env_changed("CAPSTONE_LIBRARY_PATH");
        if let Some(capstone_library_path) =
            super::get_target_env_var("CAPSTONE_LIBRARY_PATH").filter(|path| !path.is_empty())
        {
            println!("cargo:rustc-link-search=native={}", capstone_library_path);
        }
    }
}

fn add_bindings() {
    let mut builder = bindgen::Builder::default()
        .header("src/wrapper.h")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .derive_debug(true)
        .impl_debug(true)
        .constified_enum("cs_mode");
    if let Some(capstone_include_dir) =
        get_target_env_var("CAPSTONE_INCLUDE_DIR").filter(|dir| !dir.is_empty())
    {
        builder = builder.clang_arg(format!("-I{}", capstone_include_dir))
    }
    let bindings = builder.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    cargo_rerun_if_env_changed("CAPSTONE_INCLUDE_DIR");
}
