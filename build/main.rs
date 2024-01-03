#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use copy_dir::copy_dir;
use cudart_sys::{cuda_lib_path, cuda_path};
use std::env;

mod gates;
mod poseidon_constants;
mod template;

fn main() {
    let out_dir = std::path::PathBuf::from(env::var("OUT_DIR").unwrap());
    let binding = out_path.join("native");
    copy_dir("./native", binding.to_str().expect("REASON")).expect("Could not copy");
    gates::generate();
    poseidon_constants::generate();
    #[cfg(target_os = "macos")]
    std::process::exit(0);
    let dst = cmake::Config::new("native")
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
