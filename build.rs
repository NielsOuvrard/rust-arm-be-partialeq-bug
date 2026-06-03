//! # Build script for the VersatileAB Examples
//!
//! This script only executes when using `cargo` to build the project.
//!
//! Copyright (c) Ferrous Systems, 2025

use std::env;
use std::io::Write;

fn main() {
    arm_targets::process();
    write("memory.x", include_bytes!("memory.x"));

    let target = env::var("TARGET").unwrap();
    if target == "armebv7r-none-eabi" {
        write("link.x", include_bytes!("link.x")); // our patched version takes priority
    }

    // Use the cortex-m-rt linker script
    println!("cargo:rustc-link-arg=-Tlink.x");

    if target == "armebv7r-none-eabi" {
        println!("cargo:rerun-if-changed=link.x");
        println!("cargo:rustc-link-arg=-nostartfiles");
    }
}

fn write(file: &str, contents: &[u8]) {
    // Put linker file in our output directory and ensure it's on the
    // linker search path.
    let out = &std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::File::create(out.join("memory.x"))
        .unwrap()
        .write_all(contents)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed={}", file);
}
