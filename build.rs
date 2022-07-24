use std::{env, process::Command};

fn main() {
    let base_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    Command::new("cargo")
        .arg("fmt")
        .current_dir(&base_dir)
        .output()
        .unwrap();

    let rust_toolchain = env::var("RUSTUP_TOOLCHAIN").unwrap();
    if rust_toolchain.starts_with("stable") {
        // do nothing
    } else if rust_toolchain.starts_with("nightly") {
        //enable the 'nightly-features' feature flag
        println!("cargo:rustc-cfg=feature=\"nightly-features\"");
    } else {
        panic!("Unexpected value for rustc toolchain")
    }
}
