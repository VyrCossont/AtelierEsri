use glob::glob;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::vec::Vec;
use std::{env, fs};
use walkdir::WalkDir;

/// https://doc.rust-lang.org/cargo/reference/build-scripts.html
/// See https://doc.rust-lang.org/cargo/reference/build-script-examples.html
fn main() {
    aseprite_assets();
    classic_assets();
}

fn classic_assets() {
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

    w4_png2src(pngs, &dest_path);

    // Make all the generated data constants public.
    let sed_status = Command::new("sed")
        .args(&["-E", "-i", "", "-e", "s/const/pub const/g"])
        .arg(&dest_path)
        .status()
        .unwrap();
    assert!(sed_status.success());
}

/// Use WASM-4 `w4` to translate PNGs to sprites in Rust source.
fn w4_png2src<I, S, P>(inputs: I, output: P)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    P: AsRef<OsStr>,
{
    let cmd_output = Command::new("w4")
        .args(&["png2src", "--rust", "--output"])
        .arg(output)
        .args(inputs)
        .output()
        .unwrap();
    assert!(
        cmd_output.stderr.is_empty(),
        "w4 pngsrc reported errors: {}",
        std::str::from_utf8(&cmd_output.stderr).unwrap()
    );
    assert!(cmd_output.status.success());
}

struct AsepriteAssetGroup<'a> {
    name: &'a str,
    /// These can be globs.
    srcs: &'a [&'a str],
}

const ASEPRITE_ASSETS: &[AsepriteAssetGroup] = &[
    AsepriteAssetGroup {
        name: "cursor",
        srcs: &["cursor.aseprite"],
    },
    AsepriteAssetGroup {
        name: "element",
        srcs: &["element.aseprite"],
    },
    AsepriteAssetGroup {
        name: "item",
        srcs: &[
            "fantasy-tileset.aseprite",
            "roguelikeitems.aseprite",
            "items/*.aseprite",
        ],
    },
];

fn delete_dir(path: &Path) {
    if let Err(err) = fs::remove_dir_all(path) {
        if err.kind() != ErrorKind::NotFound {
            panic!(
                "Couldn't remove directory {}: {}",
                path.to_string_lossy(),
                err
            );
        }
    }
}

fn ensure_dir(path: &Path) {
    if let Err(err) = fs::create_dir_all(path) {
        if err.kind() != ErrorKind::AlreadyExists {
            panic!(
                "Couldn't create directory {}: {}",
                path.to_string_lossy(),
                err
            );
        }
    }
}

fn aseprite_export_slices(input: &Path, output_dir: &Path) {
    let status = Command::new("aseprite")
        .arg("--batch")
        .arg(input)
        .arg("--save-as")
        .arg(output_dir.join("{slice}.png"))
        .status()
        .unwrap();
    assert!(status.success());
}

fn aetools_lo5(input: &Path, output_dir: &Path) {
    let status = Command::new("aetools")
        .arg("lo5")
        .arg(input)
        .arg(output_dir)
        .status()
        .unwrap();
    assert!(status.success());
}

/// In-place `rustfmt`.
fn rustfmt(path: &Path) {
    let status = Command::new("rustfmt").arg(path).status().unwrap();
    assert!(status.success());
}

fn aseprite_assets() {
    let asset_base_dir = "asset_originals";
    println!("cargo:rerun-if-changed={asset_base_dir}");
    let asset_base_dir = Path::new(asset_base_dir);

    let build_dir = Path::new("build");
    delete_dir(build_dir);
    ensure_dir(build_dir);

    for group in ASEPRITE_ASSETS {
        let group_dir = build_dir.join(group.name);
        ensure_dir(&group_dir);

        // Export sprite slices from each Aseprite project.
        for src in group.srcs {
            for glob_result in glob(&asset_base_dir.join(src).to_string_lossy()).unwrap() {
                aseprite_export_slices(&glob_result.unwrap(), &group_dir);
            }
        }

        // Collect sprite names so we can generate structs.
        let mut sprite_names = Vec::<String>::new();

        // Take all the exported slices and lo5 them.
        let mut lo5_pngs = Vec::<OsString>::new();
        for result in glob(&group_dir.join("*.png").to_string_lossy()).unwrap() {
            let png = result.unwrap();
            let png_stem = png.file_stem().unwrap().to_string_lossy();
            sprite_names.push(png_stem.clone().into());

            aetools_lo5(&png, &group_dir);
            for suffix in ["_lo4", "_hi2"] {
                lo5_pngs.push(group_dir.join(format!("{png_stem}{suffix}.png")).into());
            }
        }

        // Generate a big file with WASM-4 Rust code for all the sprites.
        let group_rs = build_dir.join(group.name).with_extension("rs");
        w4_png2src(lo5_pngs, &group_rs);

        // Append sprite metadata structures to it.
        {
            let mut group_rs_file = File::options().append(true).open(&group_rs).unwrap();

            group_rs_file
                .write(
                    "
use crate::gfx::Lo5SplitSprite;
"
                    .as_bytes(),
                )
                .unwrap();

            for sprite_name in sprite_names {
                let cap_name = sprite_name.to_uppercase();
                group_rs_file
                    .write(
                        format!(
                            "
pub const {cap_name}: &Lo5SplitSprite = &Lo5SplitSprite {{
    w: {cap_name}_LO4_WIDTH,
    h: {cap_name}_LO4_HEIGHT,
    lo4: &{cap_name}_LO4,
    hi2: &{cap_name}_HI2,
}};
"
                        )
                        .as_bytes(),
                    )
                    .unwrap();
            }
        }

        // Make generated output readable.
        rustfmt(&group_rs);

        // Copy it into the Cargo generated-source folder.
        let out_group_rs = Path::new(&env::var_os("OUT_DIR").unwrap())
            .join(group.name)
            .with_extension("rs");
        fs::copy(group_rs, out_group_rs).unwrap();
    }
}
