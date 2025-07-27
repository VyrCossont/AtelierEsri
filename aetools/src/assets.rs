use crate::fsutil::ensure_dir;
use glob::glob;
use std::ffi::OsStr;
use std::path::Path;

/// Named asset group with source file list.
pub struct AssetGroup<'a> {
    pub name: &'a str,
    /// These can be globs.
    pub srcs: &'a [&'a str],
}

/// These images should be used as is.
/// Can accept Aseprite projects or PNGs.
pub const IMAGE_ASSETS: &[AssetGroup] = &[AssetGroup {
    name: "scene",
    srcs: &[
        "atelier_interior.aseprite",
        "new_title_screen.aseprite",
        "background-cave0.png",
    ],
}];

/// These images should be sliced and then packed into sprite sheets.
pub const SPRITE_ASSETS: &[AssetGroup] = &[
    AssetGroup {
        name: "avatar",
        srcs: &[
            "Esri.aseprite",
            "Allie.aseprite",
            "Sae.aseprite",
            "avatars/Esri_*.png",
            "avatars/Allie_*.aseprite",
            "avatars/Sae_*.png",
        ],
    },
    // AssetGroup {
    //     name: "cursor",
    //     srcs: &["cursor.aseprite"],
    // },
    AssetGroup {
        name: "element",
        srcs: &["element.aseprite"],
    },
    AssetGroup {
        name: "item",
        srcs: &[
            "fantasy-tileset.aseprite",
            "roguelikeitems.aseprite",
            "items/*.aseprite",
        ],
    },
];

/// Given a list of asset groups,
/// - create a per-group directory in the build directory
/// - for each file in each asset group, run a per-file function
/// - for each asset group, run a per-group function on the whole directory
pub fn asset_group_foreach<'a, F, G>(
    asset_groups: impl IntoIterator<Item = &'a AssetGroup<'a>>,
    asset_base_dir: &Path,
    build_dir: &Path,
    mut file_fn: F,
    mut group_fn: G,
) -> anyhow::Result<()>
where
    F: FnMut(&str, &Path, &Path, &OsStr, &str) -> anyhow::Result<()>,
    G: FnMut(&str, &Path) -> anyhow::Result<()>,
{
    for group in asset_groups {
        let group_name = group.name;
        let group_dir = build_dir.join(group_name);
        ensure_dir(&group_dir)?;
        for src_glob in group.srcs {
            for glob_result in glob(&asset_base_dir.join(src_glob).to_string_lossy())? {
                let src = glob_result?;
                let base_name = src.file_stem().ok_or(anyhow::anyhow!(
                    "Couldn't get file stem for asset file: {src}",
                    src = src.to_string_lossy()
                ))?;

                let ext = src
                    .extension()
                    .map(|ext| ext.to_string_lossy().to_string())
                    .unwrap_or("".to_string());

                file_fn(group_name, &group_dir, &src, base_name, &ext)?;
            }
        }

        group_fn(group_name, &group_dir)?;
    }
    Ok(())
}
