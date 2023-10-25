use anyhow;
use convert_case::{Case, Casing};
use glob::glob;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{BufWriter, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::Command;

type ResourceID = i16;

pub fn generate(asset_base_dir: &Path, build_dir: &Path) -> anyhow::Result<()> {
    delete_dir(build_dir)?;
    ensure_dir(build_dir)?;

    // Start at first application-usable ID that isn't in the range used for definition procedures.
    let mut pict_resource_id: ResourceID = 4096;

    let masked_pict_assets =
        generate_masked_pict_assets(asset_base_dir, build_dir, &mut pict_resource_id)?;

    let (rez, _) = generate_rez_and_header_files(build_dir, masked_pict_assets)?;

    let _ = compile_resources(build_dir, &rez)?;

    Ok(())
}

fn delete_dir(path: &Path) -> anyhow::Result<()> {
    if let Err(err) = fs::remove_dir_all(path) {
        if err.kind() != ErrorKind::NotFound {
            anyhow::bail!(
                "Couldn't remove directory {}: {}",
                path.to_string_lossy(),
                err
            );
        }
    }
    Ok(())
}

fn ensure_dir(path: &Path) -> anyhow::Result<()> {
    if let Err(err) = fs::create_dir_all(path) {
        if err.kind() != ErrorKind::AlreadyExists {
            anyhow::bail!(
                "Couldn't create directory {}: {}",
                path.to_string_lossy(),
                err
            );
        }
    }
    Ok(())
}

/// Split Aseprite sprite slices into color and mask PICT pairs.
fn generate_masked_pict_assets(
    asset_base_dir: &Path,
    build_dir: &Path,
    pict_resource_id: &mut ResourceID,
) -> anyhow::Result<Vec<(String, Vec<MaskedPictAsset>)>> {
    let mut assets = Vec::<(String, Vec<MaskedPictAsset>)>::new();

    for group in ASEPRITE_ASSETS {
        let group_name = group.name;
        let group_dir = build_dir.join(group_name);
        ensure_dir(&group_dir)?;

        // Export sprite slices from each Aseprite project.
        for aseprite_src_glob in group.srcs {
            for glob_result in glob(&asset_base_dir.join(aseprite_src_glob).to_string_lossy())? {
                let aseprite_src = glob_result?;
                aseprite_export_slices(&aseprite_src, &group_dir)?;
            }
        }

        let mut group_assets = Vec::<MaskedPictAsset>::new();

        // Convert to image and mask PICTs.
        for glob_result in glob(&group_dir.join("*.png").to_string_lossy())? {
            let png_slice = glob_result?;

            let mut image_pict = png_slice.clone();
            image_pict.set_extension("pict");
            imagemagick_convert(&png_slice, &image_pict)?;

            let mut image_pict_data = png_slice.clone();
            image_pict_data.set_extension("pictdata");
            remove_pict_header(&image_pict, &image_pict_data)?;

            let image_pict_data_rel = image_pict_data
                .strip_prefix(&build_dir)?
                .to_string_lossy()
                .to_string();

            let mut mask_pict = png_slice.clone();
            mask_pict.set_extension("mask.pict");
            imagemagick_mask(&png_slice, &mask_pict)?;

            let mut mask_pict_data = png_slice.clone();
            mask_pict_data.set_extension("mask.pictdata");
            remove_pict_header(&mask_pict, &mask_pict_data)?;

            let mask_pict_data_rel = mask_pict_data
                .strip_prefix(&build_dir)?
                .to_string_lossy()
                .to_string();

            let base_name = png_slice
                .file_stem()
                .ok_or(anyhow::anyhow!("Couldn't get file stem for PNG slice"))?
                .to_string_lossy()
                .to_string();

            let base_resource_id = *pict_resource_id;
            *pict_resource_id += 2;

            group_assets.push(MaskedPictAsset {
                base_name,
                base_resource_id,
                image_pict_data_rel,
                mask_pict_data_rel,
            })
        }

        assets.push((group_name.to_string(), group_assets));
    }

    Ok(assets)
}

struct AssetGroup<'a> {
    name: &'a str,
    /// These can be globs.
    srcs: &'a [&'a str],
}

const ASEPRITE_ASSETS: &[AssetGroup] = &[
    AssetGroup {
        name: "avatar",
        srcs: &["Esri.aseprite", "Allie.aseprite", "Sae.aseprite"],
    },
    // AssetGroup {
    //     name: "cursor",
    //     srcs: &["cursor.aseprite"],
    // },
    // AssetGroup {
    //     name: "element",
    //     srcs: &["element.aseprite"],
    // },
    AssetGroup {
        name: "item",
        srcs: &[
            "fantasy-tileset.aseprite",
            "roguelikeitems.aseprite",
            "items/*.aseprite",
        ],
    },
];

/// Export an Aseprite file to a PNG for each slice.
fn aseprite_export_slices(input: &Path, output_dir: &Path) -> anyhow::Result<()> {
    let program = "aseprite";
    let status = Command::new(program)
        .arg("--batch")
        .arg(input)
        .arg("--save-as")
        .arg(output_dir.join("{slice}.png"))
        .status()?;
    if !status.success() {
        anyhow::bail!("{program} exited with code {status}");
    }
    Ok(())
}

/// Convert an image to another format (controlled by file extensions).
fn imagemagick_convert(input: &Path, output: &Path) -> anyhow::Result<()> {
    let program = "magick";
    let status = Command::new(program).arg(input).arg(output).status()?;
    if !status.success() {
        anyhow::bail!("{program} exited with code {status}");
    }
    Ok(())
}

/// Extract an image's alpha channel as a mask image.
fn imagemagick_mask(input: &Path, output: &Path) -> anyhow::Result<()> {
    let program = "magick";
    let status = Command::new(program)
        .arg(input)
        .args(["-alpha", "extract", "-monochrome"])
        .arg(output)
        .status()?;
    if !status.success() {
        anyhow::bail!("{program} exited with code {status}");
    }
    Ok(())
}

/// Remove a PICT file's 512-byte header, which is not used when it's stored as a resource.
fn remove_pict_header(input: &Path, output: &Path) -> anyhow::Result<()> {
    let bytes = fs::read(input)?;
    if bytes.len() < 512 {
        anyhow::bail!("Underlength PICT file: {input:?}");
    }
    fs::write(output, &bytes[512..])?;
    Ok(())
}

/// Pair of image and mask PICTs.
struct MaskedPictAsset {
    base_name: String,
    base_resource_id: ResourceID,
    /// File path to headerless pict data, relative to build dir
    image_pict_data_rel: String,
    /// File path to headerless pict data, relative to build dir
    mask_pict_data_rel: String,
}

/// Write Rez resource file and headers that can be used by Rez and C.
fn generate_rez_and_header_files(
    build_dir: &Path,
    masked_pict_assets: Vec<(String, Vec<MaskedPictAsset>)>,
) -> anyhow::Result<(PathBuf, PathBuf)> {
    let rez_path = build_dir.join("Assets.r");
    let mut rez = BufWriter::new(
        File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&rez_path)?,
    );

    let header_path = build_dir.join("Assets.h");
    let mut header = BufWriter::new(
        File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&header_path)?,
    );

    write!(rez, "#include \"Assets.h\"\n\n")?;

    for (group_name, group_assets) in masked_pict_assets {
        write!(rez, "/* {group_name} */\n\n")?;
        write!(header, "/* {group_name} */\n\n")?;

        for group_asset in group_assets {
            let base_name = group_asset.base_name;

            let image_constant = format!("asset_{group_name}_{base_name}_image_pict_resource_id")
                .to_case(Case::Camel);
            write!(
                rez,
                "read 'PICT' ({image_constant}, \"{group_name} {base_name}\") \"{path}\";\n",
                path = group_asset.image_pict_data_rel,
            )?;
            write!(
                header,
                "#define {image_constant} {id}\n",
                id = group_asset.base_resource_id,
            )?;

            let mask_constant = format!("asset_{group_name}_{base_name}_mask_pict_resource_id")
                .to_case(Case::Camel);
            write!(
                rez,
                "read 'PICT' ({mask_constant}, \"{group_name} {base_name}\") \"{path}\";\n",
                path = group_asset.mask_pict_data_rel,
            )?;
            write!(
                header,
                "#define {mask_constant} {id}\n",
                id = group_asset.base_resource_id + 1,
            )?;

            write!(rez, "\n")?;
            write!(header, "\n")?;
        }

        write!(rez, "\n")?;
        write!(header, "\n")?;
    }

    Ok((rez_path, header_path))
}

/// Compile resources to a file with an actual resource fork.
/// Requires macOS and Apple's Rez (Retro68 Rez doesn't understand read statements yet).
fn compile_resources(build_dir: &Path, rez_path: &Path) -> anyhow::Result<PathBuf> {
    let rsrc_path = build_dir.join("Assets.rsrc");
    rez_compile(rez_path, build_dir, build_dir, &rsrc_path)?;
    Ok(rsrc_path)
}

/// Compile a resource file.
/// Type and creator code are for a ResEdit resource file.
fn rez_compile(
    input: &Path,
    include_path: &Path,
    resource_path: &Path,
    output: &Path,
) -> anyhow::Result<()> {
    let program = "Rez";
    let status = Command::new(program)
        .args(["-type", "rsrc", "-creator", "RSED"])
        .arg("-i")
        .arg(include_path)
        .arg("-s")
        .arg(resource_path)
        .arg("-o")
        .arg(output)
        .arg(input)
        .status()?;
    if !status.success() {
        anyhow::bail!("{program} exited with code {status}");
    }
    Ok(())
}
