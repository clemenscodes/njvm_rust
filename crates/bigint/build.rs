use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let libbigint_path = crate_dir.join("lib/bigint");
    let status = Command::new("make")
        .current_dir(&libbigint_path)
        .status()
        .expect("Failed to execute make");

    if !status.success() {
        panic!("Failed to build the library with make");
    }

    let lib_path = libbigint_path.join("build/lib");

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=static=bigint");
    println!(
        "cargo:rerun-if-changed={}",
        libbigint_path.join("build/lib/libbigint.a").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        libbigint_path.join("build/include/bigint.h").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        libbigint_path.join("build/include/support.h").display()
    );

    let bindings = bindgen::Builder::default()
        .header(
            libbigint_path
                .join("build/include/bigint.h")
                .to_string_lossy(),
        )
        .clang_arg(format!(
            "-I{}",
            libbigint_path.join("build/include").display()
        ))
        .blocklist_function("fatalError")
        .blocklist_function("newPrimObject")
        .blocklist_function("getPrimObjectDataPointer")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
