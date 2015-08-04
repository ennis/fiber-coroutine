extern crate gcc;

use std::path::PathBuf;
use std::env;

const LIB_NAME: &'static str = "libfiber.a";

fn main() {
    let arch =
        if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else {
            panic!("Unsupported architecture: {}", env::var("TARGET").unwrap());
        };
    let src_path = &["src", "asm", arch, "fiber.S"].iter().collect::<PathBuf>();
    gcc::compile_library(LIB_NAME, &[src_path.to_str().unwrap()]);

}