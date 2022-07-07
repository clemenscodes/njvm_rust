fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("outdir: {}", out_dir);
    std::process::Command::new("make")
        .args(&["-C", "lib"])
        .status()
        .unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=bigint");
}
