#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use copy_to_output::copy_to_output;
use cudart_sys::{cuda_lib_path, cuda_path};
use std::env;

mod gates;
mod poseidon_constants;
mod template;

fn main() {
    copy_to_output("native", &env::var("PROFILE").unwrap()).expect("Could not copy");
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
