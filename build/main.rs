#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use cudart_sys::{cuda_lib_path, cuda_path};
use std::env;
use std::{io, fs};

mod gates;
mod poseidon_constants;
mod template;

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    let out_path = std::path::PathBuf::from("./native");
    let out_path2 = std::path::PathBuf::from(env::var("OUT_DIR").unwrap());
    let binding2 = out_path2.join("native");
    let test = binding2.to_str().expect("REASON");
    copy_dir_all(out_path, binding2).expect("Could not copy");
    gates::generate();
    poseidon_constants::generate();
    #[cfg(target_os = "macos")]
    std::process::exit(0);
    let dst = cmake::Config::new(test)
        .profile("Release")
        .define(
            "CMAKE_CUDA_ARCHITECTURES",
            std::env::var("CUDAARCHS").unwrap_or("native".to_string()),
        )
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=boojum-cuda-native");
    println!("cargo:rustc-link-search=native={}", cuda_lib_path!());
    println!("cargo:rustc-link-lib=cudart");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=stdc++");
}
