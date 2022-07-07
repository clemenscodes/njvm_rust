use std::{env, path::PathBuf, process::Command};

use bindgen::RustTarget;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=bigint");
    Command::new("make").args(&["-C", "lib"]).status().unwrap();
    Command::new("cp")
        .args(&["lib/build/lib/libbigint.a", &out_dir])
        .status()
        .unwrap();
    let bindings = bindgen::Builder::default()
        .rust_target(RustTarget::default())
        .header("lib/build/include/bigint.h")
        .generate()
        .expect("Unable to generate bindings");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(&out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
