use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::vec::Vec;
use walkdir::WalkDir;

/// Use WASM-4 `w4` to translate PNGs to sprites in Rust source.
/// https://doc.rust-lang.org/cargo/reference/build-scripts.html
/// See https://doc.rust-lang.org/cargo/reference/build-script-examples.html
fn main() {
    println!("cargo:rerun-if-changed=assets");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("assets.rs");

    let pngs: Vec<PathBuf> = WalkDir::new("assets")
        .sort_by_file_name()
        .into_iter()
        .filter_map(|r| r.ok())
        .filter(|e| e.path().extension() == Some(OsStr::new("png")))
        .map(|e| e.into_path())
        .collect();

    let w4_png2src_status = Command::new("w4")
        .args(&["png2src", "--rust", "--output"])
        .arg(&dest_path)
        .args(&pngs)
        .status();
    assert!(w4_png2src_status.unwrap().success());

    // Make all the generated data constants public.
    let sed_status = Command::new("sed")
        .args(&["-E", "-i", "", "-e", "s/const/pub const/g"])
        .arg(&dest_path)
        .status();
    assert!(sed_status.unwrap().success());
}
