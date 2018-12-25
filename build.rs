extern crate pkg_config;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    let libs = vec!["libzfs"];
    let link_libs = vec!["nvpair"];

    for l in libs.iter() {
        pkg_config::probe_library(l).unwrap();
    }
    for l in link_libs.iter() {
        println!("cargo:rustc-link-lib={}", l);
    }
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .trust_clang_mangling(false)
        .clang_arg("-I/usr/include/libzfs")
        .clang_arg("-I/usr/include/libspl")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

}
