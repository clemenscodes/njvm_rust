use std::process::Command;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("outdir: {}", out_dir);
    Command::new("make")
        .args(&["-C", "lib"])
        .status()
        .unwrap();
    Command::new("cp")
        .args(&["lib/build/lib/libbigint.a", &out_dir])
        .status()
        .unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=bigint");
}
