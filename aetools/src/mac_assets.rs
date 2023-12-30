use anyhow;
use convert_case::{Case, Casing};
use glob::glob;
use image::{self, imageops, RgbaImage};
use png;
use rectangle_pack::{
    contains_smallest_box, pack_rects, volume_heuristic, GroupedRectsToPlace, PackedLocation,
    RectToInsert, RectanglePackError, RectanglePackOk, TargetBin,
};
use serde::Deserialize;
use serde_json;
use std::collections::{BTreeMap, HashMap};
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

    let mut masked_pict_assets =
        generate_masked_pict_assets(asset_base_dir, build_dir, &mut pict_resource_id)?;
    let (more_masked_pict_assets, rgn_assets, ninepatch_assets) =
        generate_sprite_sheet(asset_base_dir, build_dir, &mut pict_resource_id)?;
    masked_pict_assets.extend(more_masked_pict_assets);

    let (rez, _) =
        generate_rez_and_header_files(build_dir, masked_pict_assets, rgn_assets, ninepatch_assets)?;

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

/// List of named sprite locations within a sprite sheet, stored as an `RGN#` resource.
struct RGNAsset {
    resource_id: ResourceID,
    name: String,
    regions: BTreeMap<String, QDRect>,
}

impl RGNAsset {
    fn id_constant(&self) -> String {
        format!("asset_{base_name}_rgn_resource_id", base_name = self.name).to_case(Case::Camel)
    }

    fn rez(&self) -> String {
        let mut acc = Vec::<String>::new();
        acc.push(format!(
            "resource 'RGN#' ({id_constant}, \"{name}\") {{",
            name = self.name,
            id_constant = self.id_constant(),
        ));
        acc.push("    {".to_string());
        for (sprite_name, rect) in &self.regions {
            acc.push(format!("        \"{sprite_name}\","));
            acc.push(format!("        {rect},", rect = rect.rez()));
        }
        acc.push("    }".to_string());
        acc.push("};\n".to_string());
        acc.join("\n")
    }

    fn header(&self) -> String {
        // Resource ID for the region list.
        let mut acc = Vec::<String>::new();
        acc.push(format!(
            "#define {id_constant} {id}",
            id_constant = self.id_constant(),
            id = self.resource_id,
        ));
        acc.push("".to_string());

        // Indexes into the region list for each sprite.
        for (sprite_index, sprite_name) in self.regions.keys().enumerate() {
            acc.push(format!(
                "#define {id_constant} {sprite_index}",
                id_constant = format!(
                    "asset_{base_name}_{sprite_name}_sprite_index",
                    base_name = self.name
                )
                .to_case(Case::Camel),
            ));
        }
        acc.push("\n".to_string());
        acc.join("\n")
    }
}

/// List of named 9-patch locations within a sprite sheet, stored as a `9PC#` resource.
struct NinePatchAsset {
    resource_id: ResourceID,
    name: String,
    patches: BTreeMap<String, NinePatch>,
}

/// A 9-patch location.
struct NinePatch {
    /// Relative to sprite sheet origin.
    frame: QDRect,
    /// Relative to frame origin.
    center: QDRect,
}

impl NinePatchAsset {
    fn id_constant(&self) -> String {
        format!("asset_{base_name}_9pc_resource_id", base_name = self.name).to_case(Case::Camel)
    }

    fn rez(&self) -> String {
        let mut acc = Vec::<String>::new();
        acc.push(format!(
            "resource '9PC#' ({id_constant}, \"{name}\") {{",
            name = self.name,
            id_constant = self.id_constant(),
        ));
        acc.push("    {".to_string());
        for (sprite_name, NinePatch { frame, center }) in &self.patches {
            acc.push(format!("        \"{sprite_name}\","));
            acc.push(format!("        {frame},", frame = frame.rez()));
            acc.push(format!("        {center},", center = center.rez()));
        }
        acc.push("    }".to_string());
        acc.push("};\n".to_string());
        acc.join("\n")
    }

    fn header(&self) -> String {
        // Resource ID for the patch list.
        let mut acc = Vec::<String>::new();
        acc.push(format!(
            "#define {id_constant} {id}",
            id_constant = self.id_constant(),
            id = self.resource_id,
        ));
        acc.push("".to_string());

        // Indexes into the patch list for each sprite.
        for (sprite_index, sprite_name) in self.patches.keys().enumerate() {
            acc.push(format!(
                "#define {id_constant} {sprite_index}",
                id_constant = format!(
                    "asset_{base_name}_{sprite_name}_9patch_index",
                    base_name = self.name
                )
                .to_case(Case::Camel),
            ));
        }
        acc.push("\n".to_string());
        acc.join("\n")
    }
}

/// QuickDraw `RECT`.
#[derive(Debug, Clone)]
struct QDRect {
    top: i16,
    left: i16,
    bottom: i16,
    right: i16,
}

impl QDRect {
    fn rez(&self) -> String {
        format!(
            "{{{top}, {left}, {bottom}, {right}}}",
            top = self.top,
            left = self.left,
            bottom = self.bottom,
            right = self.right
        )
    }
}

impl TryFrom<&AsepriteRect> for QDRect {
    type Error = anyhow::Error;

    fn try_from(value: &AsepriteRect) -> Result<Self, Self::Error> {
        let x = i16::try_from(value.x)?;
        let y = i16::try_from(value.y)?;
        let w = i16::try_from(value.w)?;
        let h = i16::try_from(value.h)?;
        Ok(Self {
            top: y,
            left: x,
            bottom: y + h,
            right: x + w,
        })
    }
}

/// Top-level sprite info JSON for an Aseprite project.
///
/// https://www.aseprite.org/docs/cli#data
#[derive(Debug, Deserialize)]
struct AsepriteProject {
    meta: AsepriteMeta,
}

#[derive(Debug, Deserialize)]
struct AsepriteMeta {
    slices: Vec<AsepriteSlice>,
}

#[derive(Debug, Deserialize)]
struct AsepriteSlice {
    name: String,
    keys: Vec<AsepriteSliceKey>,
}

#[derive(Debug, Deserialize)]
struct AsepriteSliceKey {
    bounds: AsepriteRect,
    /// 9-patch data. Origin relative to bounds.
    center: Option<AsepriteRect>,
}

#[derive(Debug, Deserialize)]
struct AsepriteRect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

/// Combine all Aseprite sprite slices into a single color and mask PICT pair.
fn generate_sprite_sheet(
    asset_base_dir: &Path,
    build_dir: &Path,
    pict_resource_id: &mut ResourceID,
) -> anyhow::Result<(
    Vec<(String, Vec<MaskedPictAsset>)>,
    Vec<RGNAsset>,
    Vec<NinePatchAsset>,
)> {
    let mut masked_pict_assets = Vec::<(String, Vec<MaskedPictAsset>)>::new();
    // There's only one output group.
    let mut group_assets = Vec::<MaskedPictAsset>::new();
    let mut rgn_assets = Vec::<RGNAsset>::new();
    let mut ninepatch_assets = Vec::<NinePatchAsset>::new();

    // Map of input-group-qualified sprite name to sprite path.
    let mut sprite_paths = HashMap::<String, PathBuf>::new();
    let mut rects_to_place = GroupedRectsToPlace::<String, ()>::new();

    // Map of input-group-qualified sprite name to 9-patch center rect, if it has one.
    let mut ninepatch_centers = HashMap::<String, QDRect>::new();

    for group in ASEPRITE_SPRITE_ASSETS {
        let group_name = group.name;
        let group_dir = build_dir.join(group_name);
        ensure_dir(&group_dir)?;

        // Export sprite slices from each Aseprite project.
        for aseprite_src_glob in group.srcs {
            for glob_result in glob(&asset_base_dir.join(aseprite_src_glob).to_string_lossy())? {
                let aseprite_src = glob_result?;
                aseprite_export_slices(&aseprite_src, &group_dir)?;

                // Get sprite metadata to identify sprites that are 9-patches.
                let aseprite_project = {
                    let base_name = aseprite_src
                        .file_stem()
                        .ok_or(anyhow::anyhow!("Couldn't get file stem for Aseprite file"))?;
                    let mut metadata_json = group_dir.join(base_name);
                    metadata_json.set_extension("json");
                    aseprite_export_metadata(&aseprite_src, &metadata_json)?;
                    aseprite_read_metadata(&metadata_json)?
                };
                for slice in &aseprite_project.meta.slices {
                    if slice.keys.len() != 1 {
                        anyhow::bail!("Expected exactly one keyframe per slice");
                    }
                    if let Some(center) = &slice.keys[0].center {
                        let sprite_name =
                            format!("{group_name}_{base_name}", base_name = slice.name);
                        ninepatch_centers.insert(sprite_name, center.try_into()?);
                    }
                }
            }
        }

        // Collect metadata for each sprite.
        for glob_result in glob(&group_dir.join("*.png").to_string_lossy())? {
            let png_slice = glob_result?;

            let decoder = png::Decoder::new(File::open(&png_slice)?);
            let reader = decoder.read_info()?;

            let info = reader.info();

            let base_name = png_slice
                .file_stem()
                .ok_or(anyhow::anyhow!("Couldn't get file stem for PNG slice"))?
                .to_string_lossy()
                .to_string();

            let sprite_name = format!("{group_name}_{base_name}");

            sprite_paths.insert(sprite_name.clone(), png_slice);
            rects_to_place.push_rect(
                sprite_name,
                None,
                RectToInsert::new(info.width, info.height, 1),
            );
        }
    }

    // Place rectangles in as many sprite sheets as necessary.
    // Does not currently take asset groups into account.
    // `RGN#` and `9PC#` resources will be assigned the same IDs as the image `PICT` resource.
    let mut target_bins = BTreeMap::new();
    // Arbitrary size.
    let sheet_w = 256u32;
    let sheet_h = 256u32;
    let max_sheet_count = 1;
    let rectangle_placements: RectanglePackOk<String, ResourceID>;
    loop {
        target_bins.insert(*pict_resource_id, TargetBin::new(sheet_w, sheet_h, 1));
        match pack_rects(
            &rects_to_place,
            &mut target_bins,
            &volume_heuristic,
            &contains_smallest_box,
        ) {
            Err(e) => match e {
                RectanglePackError::NotEnoughBinSpace => {
                    if target_bins.len() < max_sheet_count {
                        *pict_resource_id += 2;
                        continue;
                    } else {
                        anyhow::bail!(
                            "Hit max sheet count {max_sheet_count} while packing sprites!"
                        );
                    }
                }
            },
            Ok(placements) => {
                rectangle_placements = placements;
                break;
            }
        }
    }
    let mut sprites_for_sheet = BTreeMap::<ResourceID, BTreeMap<String, PackedLocation>>::new();
    for (sprite_name, (base_resource_id, location)) in rectangle_placements.packed_locations() {
        if let Some(sprites) = sprites_for_sheet.get_mut(base_resource_id) {
            sprites.insert(sprite_name.clone(), location.clone());
        } else {
            let mut sprites = BTreeMap::<String, PackedLocation>::new();
            sprites.insert(sprite_name.clone(), location.clone());
            sprites_for_sheet.insert(*base_resource_id, sprites);
        }
    }

    let sprite_sheets_dir = build_dir.join("sprite_sheet");
    ensure_dir(&sprite_sheets_dir)?;
    for (sheet_number, (base_resource_id, sprites)) in sprites_for_sheet.iter().enumerate() {
        // Copy all the sprites into a sheet PNG.
        let sprite_sheet_png = sprite_sheets_dir.join(format!("{sheet_number:02}.png"));
        // Assume 8 bits per channel is enough.
        let mut sprite_sheet = RgbaImage::new(sheet_w, sheet_h);
        for (sprite_name, location) in sprites {
            let sprite = image::open(&sprite_paths[sprite_name])?;
            imageops::replace(
                &mut sprite_sheet,
                &sprite,
                location.x() as i64,
                location.y() as i64,
            );
        }
        sprite_sheet.save(&sprite_sheet_png)?;

        group_assets.push(png_to_pict(
            build_dir,
            format!("{sheet_number:02}"),
            *base_resource_id,
            &sprite_sheet_png,
        )?);

        let mut rgn_sprites = BTreeMap::<String, QDRect>::new();
        let mut ninepatch_sprites = BTreeMap::<String, NinePatch>::new();

        for (sprite_name, location) in sprites {
            let x = i16::try_from(location.x())?;
            let y = i16::try_from(location.y())?;
            let w = i16::try_from(location.width())?;
            let h = i16::try_from(location.height())?;
            let frame = QDRect {
                top: y,
                left: x,
                bottom: y + h,
                right: x + w,
            };
            if let Some(center) = ninepatch_centers.get(sprite_name) {
                ninepatch_sprites.insert(
                    sprite_name.clone(),
                    NinePatch {
                        frame,
                        center: center.clone(),
                    },
                );
            } else {
                rgn_sprites.insert(sprite_name.clone(), frame);
            }
        }

        rgn_assets.push(RGNAsset {
            resource_id: *base_resource_id,
            name: format!("sprite_sheet {sheet_number:02}"),
            regions: rgn_sprites,
        });

        ninepatch_assets.push(NinePatchAsset {
            resource_id: *base_resource_id,
            name: format!("sprite_sheet {sheet_number:02}"),
            patches: ninepatch_sprites,
        });
    }

    masked_pict_assets.push(("sprite_sheet".to_string(), group_assets));

    Ok((masked_pict_assets, rgn_assets, ninepatch_assets))
}

/// Convert a PNG to image and mask PICTs.
fn png_to_pict(
    build_dir: &Path,
    base_name: String,
    base_resource_id: ResourceID,
    png: &Path,
) -> anyhow::Result<MaskedPictAsset> {
    let mut image_pict = png.to_path_buf();
    image_pict.set_extension("pict");
    imagemagick_convert(&png, &image_pict)?;

    let mut image_pict_data = png.to_path_buf();
    image_pict_data.set_extension("pictdata");
    remove_pict_header(&image_pict, &image_pict_data)?;

    let image_pict_data_rel = image_pict_data
        .strip_prefix(&build_dir)?
        .to_string_lossy()
        .to_string();

    let mut mask_pict = png.to_path_buf();
    mask_pict.set_extension("mask.pict");
    imagemagick_mask(&png, &mask_pict)?;

    let mut mask_pict_data = png.to_path_buf();
    mask_pict_data.set_extension("mask.pictdata");
    remove_pict_header(&mask_pict, &mask_pict_data)?;

    let mask_pict_data_rel = mask_pict_data
        .strip_prefix(&build_dir)?
        .to_string_lossy()
        .to_string();

    Ok(MaskedPictAsset {
        base_name,
        base_resource_id,
        image_pict_data_rel,
        mask_pict_data_rel,
    })
}

/// Split Aseprite sprite slices into color and mask PICT pairs.
fn generate_masked_pict_assets(
    asset_base_dir: &Path,
    build_dir: &Path,
    pict_resource_id: &mut ResourceID,
) -> anyhow::Result<Vec<(String, Vec<MaskedPictAsset>)>> {
    let mut assets = Vec::<(String, Vec<MaskedPictAsset>)>::new();

    for group in ASEPRITE_IMAGE_ASSETS {
        let group_name = group.name;
        let group_dir = build_dir.join(group_name);
        ensure_dir(&group_dir)?;

        // Export image from each Aseprite project.
        for aseprite_src_glob in group.srcs {
            for glob_result in glob(&asset_base_dir.join(aseprite_src_glob).to_string_lossy())? {
                let aseprite_src = glob_result?;
                let base_name = aseprite_src.file_stem().ok_or(anyhow::anyhow!(
                    "Couldn't get file stem for Aseprite project"
                ))?;
                let mut image_png = group_dir.join(base_name);
                image_png.set_extension("png");
                aseprite_export(&aseprite_src, &image_png)?;
            }
        }

        let mut group_assets = Vec::<MaskedPictAsset>::new();

        // Convert to image and mask PICTs.
        for glob_result in glob(&group_dir.join("*.png").to_string_lossy())? {
            let image_png = glob_result?;

            let base_name = image_png
                .file_stem()
                .ok_or(anyhow::anyhow!("Couldn't get file stem for PNG image"))?
                .to_string_lossy()
                .to_string();

            let base_resource_id = *pict_resource_id;
            *pict_resource_id += 2;

            group_assets.push(png_to_pict(
                build_dir,
                base_name,
                base_resource_id,
                &image_png,
            )?);
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

/// These images should be used as is.
const ASEPRITE_IMAGE_ASSETS: &[AssetGroup] = &[AssetGroup {
    name: "scene",
    srcs: &["atelier_interior.aseprite", "new_title_screen.aseprite"],
}];

/// These images should be sliced and then packed into sprite sheets.
const ASEPRITE_SPRITE_ASSETS: &[AssetGroup] = &[
    AssetGroup {
        name: "avatar",
        srcs: &["Esri.aseprite", "Allie.aseprite", "Sae.aseprite"],
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

/// Export an Aseprite file to a single image.
fn aseprite_export(input: &Path, output: &Path) -> anyhow::Result<()> {
    let program = "aseprite";
    let status = Command::new(program)
        .arg("--batch")
        .arg(input)
        .arg("--save-as")
        .arg(output)
        .status()?;
    if !status.success() {
        anyhow::bail!("{program} exited with code {status}");
    }
    Ok(())
}

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

/// Export sprite metadata from an Aseprite file.
fn aseprite_export_metadata(input: &Path, output: &Path) -> anyhow::Result<()> {
    let program = "aseprite";
    let status = Command::new(program)
        .arg("--batch")
        .arg("--list-slices")
        .arg(input)
        .arg("--data")
        .arg(output)
        .status()?;
    if !status.success() {
        anyhow::bail!("{program} exited with code {status}");
    }
    Ok(())
}

fn aseprite_read_metadata(input: &Path) -> anyhow::Result<AsepriteProject> {
    let aseprite_project = serde_json::from_reader(File::open(input)?)?;
    Ok(aseprite_project)
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
/// Note that masks for QuickDraw `CopyMask` are inverted: black pixels are copied, white pixels are ignored.
fn imagemagick_mask(input: &Path, output: &Path) -> anyhow::Result<()> {
    let program = "magick";
    let status = Command::new(program)
        .arg(input)
        .args(["-alpha", "extract", "-monochrome", "-negate"])
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

/// A 32×32 B&W icon. Does not support a mask.
struct ICONAsset {
    /// File path to bare icon data, relative to build dir
    data_rel: String,
    /// Allocate resource ID from range usable by Menu Manager:
    /// https://preterhuman.net/macstuff/insidemac/Toolbox/Toolbox-101.html#HEADING101-48
    menu: bool,
}

/// A 16×16 B&W icon with optional mask.
/// Technically the mask is the second entry in a list of icons,
/// but no Toolbox routines seem to use anything past the mask.
struct SICNAsset {
    /// File path to bare icon data, relative to build dir
    data_rel: String,
    /// Allocate resource ID from range usable by Menu Manager.
    menu: bool,
}

/// Write Rez resource file and headers that can be used by Rez and C.
fn generate_rez_and_header_files(
    build_dir: &Path,
    masked_pict_assets: Vec<(String, Vec<MaskedPictAsset>)>,
    rgn_assets: Vec<RGNAsset>,
    ninepatch_assets: Vec<NinePatchAsset>,
) -> anyhow::Result<(PathBuf, PathBuf)> {
    // Copy custom resource types file as is.
    {
        let aetypes_path = build_dir.join("AETypes.r");
        let mut aetypes = BufWriter::new(
            File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&aetypes_path)?,
        );
        aetypes.write(include_bytes!("AETypes.r"))?;
    }

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

    write!(rez, "#include \"AETypes.r\"\n")?;
    write!(rez, "#include \"Assets.h\"\n")?;
    write!(rez, "\n")?;

    write!(header, "#ifndef ASSETS_H\n")?;
    write!(header, "#define ASSETS_H\n")?;
    write!(header, "\n")?;

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

    for rgn_asset in rgn_assets {
        write!(rez, "/* sprite sheet region lists */\n\n")?;
        write!(header, "/* sprite sheet region lists */\n\n")?;

        write!(rez, "{src}", src = rgn_asset.rez())?;
        write!(header, "{src}", src = rgn_asset.header())?;

        write!(rez, "\n")?;
        write!(header, "\n")?;
    }

    for ninepatch_asset in ninepatch_assets {
        write!(rez, "/* sprite sheet 9-patch lists */\n\n")?;
        write!(header, "/* sprite sheet 9-patch lists */\n\n")?;

        write!(rez, "{src}", src = ninepatch_asset.rez())?;
        write!(header, "{src}", src = ninepatch_asset.header())?;

        write!(rez, "\n")?;
        write!(header, "\n")?;
    }

    write!(header, "#endif /* ASSETS_H */\n")?;

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
