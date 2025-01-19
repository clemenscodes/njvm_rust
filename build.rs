fn main() {
    let libbigint_path = "lib/bigint";

    let status = std::process::Command::new("make")
        .current_dir(libbigint_path)
        .status()
        .expect("Failed to execute make");

    if !status.success() {
        panic!("Failed to build the library with make");
    }

    println!("cargo:rustc-link-search=native=lib/bigint/build/lib");
    println!("cargo:rustc-link-lib=static=bigint");
    println!("cargo:rerun-if-changed=lib/bigint/build/lib/libbigint.a");
    println!("cargo:rerun-if-changed=lib/bigint/build/include/bigint.h");
    println!("cargo:rerun-if-changed=lib/bigint/build/include/support.h");

    let bindings = bindgen::Builder::default()
        .header("lib/bigint/build/include/bigint.h")
        .clang_arg("-Ilib/bigint/build/include")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
