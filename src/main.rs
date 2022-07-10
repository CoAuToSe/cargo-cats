#![recursion_limit = "256"]
use std::{
    env::{self, *},
    fs,
    path::{self, *},
};
macro_rules! show {
    ($a:expr) => {
        println!("{:?}", $a);
};
    ($a:expr, $($b:expr),+) => {
        print!("{}", $a);
        show!($($b),+);
    };
}

fn main() {
    #[cfg(feature = "env-var")]
    {
        println!("Hello, world!");
        show!("Hello, world!");
        show!(current_dir());
        show!(current_exe());
        show!(args_os());
        show!(args());
        let paths = [Path::new("/bin"), Path::new("/usr/bin")];
        show!(join_paths(paths.iter()));
        println!();

        let key = "PATH";
        match env::var_os(key) {
            Some(paths) => {
                for path in env::split_paths(&paths) {
                    println!("'{}'", path.display());
                }
            }
            None => println!("{} is not defined in the environment.", key),
        }
        println!();
        let key = "HOME";
        match env::var_os(key) {
            Some(val) => println!("{}: {:?}", key, val),
            None => println!("{} is not defined in the environment.", key),
        }
        println!();
        for (key, value) in env::vars_os() {
            println!("{:?}: {:?}", key, value);
        }
        println!();
        for (key, value) in env::vars() {
            println!("{}: {}", key, value);
        }

        println!();
        let mut dir = env::temp_dir();
        println!("Temporary directory: {}", dir.display());
    }
    #[cfg(feature = "build")]
    {
        println!("{:?}", env::var_os("CARGO_CRATE_NAME").unwrap());
        println!("{:?}", env::var_os("OUT_DIR").unwrap());
        let out_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("build.rs");
        if !dest_path.exists() {
            fs::write(
                &dest_path,
                r#"//! Generated by cargo-cats using "cargo cats".

use std::{env, process::Command};

fn main() {
    let base_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    Command::new("cargo")
        .arg("fmt")
        .current_dir(&base_dir)
        .output()
        .unwrap();
}"#,
            )
            .unwrap();
        }
    }
}
