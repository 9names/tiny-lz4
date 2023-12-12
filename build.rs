use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("lz4.s"))
        .unwrap()
        .write_all(include_bytes!("src/lz4.s"))
        .unwrap();
    Command::new("arm-none-eabi-as")
        .args(["lz4.s", "-o", "liblz4.o"])
        .current_dir(out)
        .status()
        .unwrap();
    Command::new("arm-none-eabi-ar")
        .args(["-rcs", "liblz4.a", "liblz4.o"])
        .current_dir(out)
        .status()
        .unwrap();
    println!("cargo:rustc-link-search=native={}", out.to_string_lossy());
    println!("cargo:rustc-link-lib=static=lz4");
    println!("cargo:rerun-if-changed=src/lz4.s");
}
