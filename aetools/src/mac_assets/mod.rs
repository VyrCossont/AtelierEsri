mod cinematic;
mod tiled;

use crate::assets::{asset_group_foreach, IMAGE_ASSETS, SPRITE_ASSETS};
use crate::ext::{aseprite, imagemagick};
use crate::fsutil::{delete_dir, ensure_dir};
use crate::mac::resource::TypedResource;
use crate::mac::OSType;
use crate::mac_assets::cinematic::compile_cinematics;
use crate::mac_assets::tiled::{compile_maps, TMXAsset, TSXAsset};
use aetools_derive::cpp_codegen;
use anyhow;
use convert_case::{Case, Casing};
use glob::glob;
use image::{self, image_dimensions, imageops, RgbaImage};
use png;
use rectangle_pack::{
    contains_smallest_box, pack_rects, volume_heuristic, GroupedRectsToPlace, PackedLocation,
    RectToInsert, RectanglePackError, RectanglePackOk, TargetBin,
};
use std::collections::{BTreeMap, HashMap};
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

type ResourceID = i16;

/// See https://preterhuman.net/macstuff/insidemac/MoreToolbox/MoreToolbox-27.html#MARKER-9-196
#[derive(Debug, Default)]
pub struct ResourceIDGenerator {
    /// Stores the next ID to be returned for that resource type.
    map: BTreeMap<OSType, ResourceID>,
}

impl ResourceIDGenerator {
    const FIRST_ID: ResourceID = 128;

    /// Get an ID for that resource type.
    fn get(&mut self, os_type: OSType) -> ResourceID {
        if let Some(map_id) = self.map.get_mut(&os_type) {
            let id = *map_id;
            *map_id += 1;
            return id;
        }
        self.map.insert(os_type, Self::FIRST_ID + 1);
        return Self::FIRST_ID;
    }
}

// TODO: consider extracting asset name and resource ID into a `RezMeta` type, which could also include resource flags.
/// Can emit itself as Rez source and C headers.
trait Resourceful: TypedResource {
    fn name(&self) -> String;

    /// May collide with other resource types, or be empty, in which case, override this.
    fn id_safe_os_type() -> String {
        Self::OS_TYPE
            .iter()
            .filter_map(|c| {
                let c = *c as char;
                match c {
                    'a'..='z' | '0'..='9' => Some(c),
                    'A'..='Z' => Some(c.to_ascii_lowercase()),
                    _ => None,
                }
            })
            .collect()
    }

    /// Hack: OSTypes are assumed to be *MacRoman* when treated as strings, not UTF-8,
    /// but to keep things simple, we will only use ASCII in our own resource types.
    /// TODO: refactor Rez methods to work in MacRoman
    fn os_type_rez() -> String {
        for c in Self::OS_TYPE {
            if !c.is_ascii() {
                panic!("OSType can't contain non-ASCII characters");
            }
            if c.is_ascii_control() {
                panic!("OSType shouldn't include control characters");
            }
            if c == b'\'' {
                // TODO: figure out how to escape things in Rez source
                panic!("OSType shouldn't include apostrophes");
            }
        }
        String::from_utf8(Self::OS_TYPE.to_vec()).unwrap_or("OSType is not valid UTF-8".to_string())
    }

    fn id_constant(&self) -> String {
        format!(
            "asset_{name}_{os_type}_resource_id",
            os_type = Self::id_safe_os_type(),
            name = self.name()
        )
        .to_case(Case::Camel)
    }

    fn rez(&self) -> String;

    fn header(&self) -> String;
}

pub fn generate(asset_base_dir: &Path, build_dir: &Path) -> anyhow::Result<()> {
    delete_dir(build_dir)?;
    ensure_dir(build_dir)?;

    // Start at first application-usable ID that isn't in the range used for definition procedures.
    // TODO: which things even care about definition procedures?
    let mut resource_id_generator = ResourceIDGenerator::default();

    let mut masked_pict_asset_groups =
        generate_masked_pict_assets(asset_base_dir, build_dir, &mut resource_id_generator)?;

    let mut rgn_asset_groups = Vec::<(String, Vec<RGNAsset>)>::new();

    let (sprite_sheet_masked_pict_asset_groups, sprite_sheet_rgn_assets, ninepatch_assets) =
        generate_sprite_sheet(asset_base_dir, build_dir, &mut resource_id_generator)?;
    masked_pict_asset_groups.extend(sprite_sheet_masked_pict_asset_groups);
    rgn_asset_groups.push(("sprite_sheet".to_string(), sprite_sheet_rgn_assets.clone()));

    let (map_masked_pict_assets, tsx_assets, map_rgn_assets, tmx_assets) =
        compile_maps(asset_base_dir, build_dir, &mut resource_id_generator)?;
    masked_pict_asset_groups.push(("tileset".to_string(), map_masked_pict_assets));
    rgn_asset_groups.push(("map".to_string(), map_rgn_assets));

    let (rez, _) = generate_rez_and_header_files(
        build_dir,
        &masked_pict_asset_groups,
        &rgn_asset_groups,
        &ninepatch_assets,
        &tsx_assets,
        &tmx_assets,
    )?;

    let _ = compile_resources(build_dir, &rez)?;

    let _ = compile_cinematics(
        asset_base_dir,
        &masked_pict_asset_groups,
        &sprite_sheet_rgn_assets,
        build_dir,
    )?;

    Ok(())
}

/// List of named sprite locations within a sprite sheet, stored as an `RGN#` resource.
#[derive(Debug, Clone)]
pub struct RGNAsset {
    pub resource_id: ResourceID,
    pub name: String,
    pub regions: BTreeMap<String, QDRect>,
}

impl RGNAsset {
    fn new(
        resource_id_generator: &mut ResourceIDGenerator,
        name: String,
        regions: BTreeMap<String, QDRect>,
    ) -> Self {
        let resource_id = resource_id_generator.get(Self::OS_TYPE);
        Self {
            resource_id,
            name,
            regions,
        }
    }
}

impl TypedResource for RGNAsset {
    const OS_TYPE: OSType = *b"RGN#";
}

impl Resourceful for RGNAsset {
    fn name(&self) -> String {
        return self.name.clone();
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

/// A 9-patch location.
struct NinePatch {
    /// Relative to sprite sheet origin.
    frame: QDRect,
    /// Relative to frame origin.
    center: QDRect,
}

/// List of named 9-patch locations within a sprite sheet, stored as a `9PC#` resource.
struct NinePatchAsset {
    resource_id: ResourceID,
    name: String,
    patches: BTreeMap<String, NinePatch>,
}

impl NinePatchAsset {
    fn new(
        resource_id_generator: &mut ResourceIDGenerator,
        name: String,
        patches: BTreeMap<String, NinePatch>,
    ) -> Self {
        let resource_id = resource_id_generator.get(Self::OS_TYPE);
        Self {
            resource_id,
            name,
            patches,
        }
    }
}

impl TypedResource for NinePatchAsset {
    const OS_TYPE: OSType = *b"9PC#";
}

impl Resourceful for NinePatchAsset {
    fn name(&self) -> String {
        self.name.clone()
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
#[cpp_codegen]
pub struct QDRect {
    pub top: i16,
    pub left: i16,
    pub bottom: i16,
    pub right: i16,
}

pub fn hpp() -> anyhow::Result<()> {
    println!("{hpp}", hpp = QDRect::hpp());
    Ok(())
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

impl TryFrom<&aseprite::Rect> for QDRect {
    type Error = anyhow::Error;

    fn try_from(value: &aseprite::Rect) -> Result<Self, Self::Error> {
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

/// Combine all Aseprite sprite slices into a single color and mask PICT pair.
fn generate_sprite_sheet(
    asset_base_dir: &Path,
    build_dir: &Path,
    resource_id_generator: &mut ResourceIDGenerator,
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

    let export_or_copy_sprites = |group_name: &str,
                                  group_dir: &Path,
                                  src: &Path,
                                  base_name: &OsStr,
                                  ext: &str|
     -> anyhow::Result<()> {
        match ext {
            "aseprite" => {
                // Export sprite slices from each Aseprite project into the group directory.
                aseprite::export_slices(&src, &group_dir)?;

                // Get sprite metadata to identify sprites that are 9-patches.
                let aseprite_project = {
                    let mut metadata_json = group_dir.join(base_name);
                    metadata_json.set_extension("json");
                    aseprite::export_metadata(&src, &metadata_json)?;
                    aseprite::read_metadata(&metadata_json)?
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
            "png" => {
                // Copy PNG sprites into the group directory.
                let mut image_png = group_dir.join(base_name);
                image_png.set_extension("png");
                fs::copy(src, image_png)?;
            }
            _ => anyhow::bail!("Unsupported file extension: {ext}"),
        }
        Ok(())
    };

    let collect_sprite_metadata = |group_name: &str, group_dir: &Path| -> anyhow::Result<()> {
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
        Ok(())
    };

    asset_group_foreach(
        SPRITE_ASSETS,
        asset_base_dir,
        build_dir,
        export_or_copy_sprites,
        collect_sprite_metadata,
    )?;

    // Place rectangles in as many sprite sheets as necessary.
    // Does not currently take asset groups into account.
    let mut target_bins = BTreeMap::new();
    // Arbitrary size.
    let sheet_w = 512u32;
    let sheet_h = 256u32;
    let max_sheet_count = 1;
    let rectangle_placements: RectanglePackOk<String, usize>;
    {
        let mut sheet_number = 0usize;
        loop {
            target_bins.insert(sheet_number, TargetBin::new(sheet_w, sheet_h, 1));
            match pack_rects(
                &rects_to_place,
                &mut target_bins,
                &volume_heuristic,
                &contains_smallest_box,
            ) {
                Err(e) => match e {
                    RectanglePackError::NotEnoughBinSpace => {
                        if target_bins.len() < max_sheet_count {
                            sheet_number += 1;
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
    }

    // Build a map of the sheet and location assigned to each sprite.
    let mut sprites_for_sheet = BTreeMap::<usize, BTreeMap<String, PackedLocation>>::new();
    for (sprite_name, (sheet_number, location)) in rectangle_placements.packed_locations() {
        if let Some(sprites) = sprites_for_sheet.get_mut(sheet_number) {
            sprites.insert(sprite_name.clone(), location.clone());
        } else {
            let mut sprites = BTreeMap::<String, PackedLocation>::new();
            sprites.insert(sprite_name.clone(), location.clone());
            sprites_for_sheet.insert(*sheet_number, sprites);
        }
    }

    let sprite_sheets_dir = build_dir.join("sprite_sheet");
    ensure_dir(&sprite_sheets_dir)?;
    for (sheet_number, sprites) in sprites_for_sheet.iter() {
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
            resource_id_generator,
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

        rgn_assets.push(RGNAsset::new(
            resource_id_generator,
            format!("sprite_sheet {sheet_number:02}"),
            rgn_sprites,
        ));

        ninepatch_assets.push(NinePatchAsset::new(
            resource_id_generator,
            format!("sprite_sheet {sheet_number:02}"),
            ninepatch_sprites,
        ));
    }

    masked_pict_assets.push(("sprite_sheet".to_string(), group_assets));

    Ok((masked_pict_assets, rgn_assets, ninepatch_assets))
}

/// Convert a PNG to image and mask PICTs.
pub fn png_to_pict(
    build_dir: &Path,
    base_name: String,
    resource_id_generator: &mut ResourceIDGenerator,
    png: &Path,
) -> anyhow::Result<MaskedPictAsset> {
    let pict_os_type: OSType = *b"PICT";
    let image_pict_resource_id = resource_id_generator.get(pict_os_type);

    let (image_width, image_height) = image_dimensions(png)?;
    let image_width = i16::try_from(image_width)?;
    let image_height = i16::try_from(image_height)?;

    let mut image_pict = png.to_path_buf();
    image_pict.set_extension("pict");
    imagemagick::convert(&png, &image_pict)?;

    let mut image_pict_data = png.to_path_buf();
    image_pict_data.set_extension("pictdata");
    remove_pict_header(&image_pict, &image_pict_data)?;

    let image_pict_data_rel = image_pict_data
        .strip_prefix(&build_dir)?
        .to_string_lossy()
        .to_string();

    let (mask_pict_resource_id, mask_pict_data_rel) = if imagemagick::opaque(png)? {
        (None, None)
    } else {
        let mut mask_pict = png.to_path_buf();
        mask_pict.set_extension("mask.pict");
        imagemagick::mask(&png, &mask_pict, true)?;

        let mut mask_pict_data = png.to_path_buf();
        mask_pict_data.set_extension("mask.pictdata");
        remove_pict_header(&mask_pict, &mask_pict_data)?;

        (
            Some(resource_id_generator.get(pict_os_type)),
            Some(
                mask_pict_data
                    .strip_prefix(&build_dir)?
                    .to_string_lossy()
                    .to_string(),
            ),
        )
    };

    Ok(MaskedPictAsset {
        base_name,
        image_width,
        image_height,
        image_pict_resource_id,
        image_pict_data_rel,
        mask_pict_resource_id,
        mask_pict_data_rel,
    })
}

/// Split Aseprite sprite slices into color and mask PICT pairs.
fn generate_masked_pict_assets(
    asset_base_dir: &Path,
    build_dir: &Path,
    resource_id_generator: &mut ResourceIDGenerator,
) -> anyhow::Result<Vec<(String, Vec<MaskedPictAsset>)>> {
    let mut assets = Vec::<(String, Vec<MaskedPictAsset>)>::new();

    let export_or_copy_images = |_group_name: &str,
                                 group_dir: &Path,
                                 src: &Path,
                                 base_name: &OsStr,
                                 ext: &str|
     -> anyhow::Result<()> {
        let mut image_png = group_dir.join(base_name);
        image_png.set_extension("png");

        match ext {
            "aseprite" => aseprite::export(&src, &image_png)?,
            "png" => {
                fs::copy(src, image_png)?;
            }
            _ => anyhow::bail!("Unsupported file extension: {ext}"),
        }
        Ok(())
    };

    let convert_pngs_to_picts = |group_name: &str, group_dir: &Path| -> anyhow::Result<()> {
        let mut group_assets = Vec::<MaskedPictAsset>::new();

        // Convert to image and mask PICTs.
        for glob_result in glob(&group_dir.join("*.png").to_string_lossy())? {
            let image_png = glob_result?;

            let base_name = image_png
                .file_stem()
                .ok_or(anyhow::anyhow!("Couldn't get file stem for PNG image"))?
                .to_string_lossy()
                .to_string();

            let asset = png_to_pict(build_dir, base_name, resource_id_generator, &image_png)?;

            group_assets.push(asset);
        }

        assets.push((group_name.to_string(), group_assets));

        Ok(())
    };

    asset_group_foreach(
        IMAGE_ASSETS,
        asset_base_dir,
        build_dir,
        export_or_copy_images,
        convert_pngs_to_picts,
    )?;

    Ok(assets)
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

// TODO: give image/mask pairs and sprite sheets their own resource types so that the game only needs to know one ID
/// Pair of image and mask PICTs.
/// Mask is optional.
#[derive(Debug, Clone)]
pub struct MaskedPictAsset {
    pub base_name: String,
    pub image_width: i16,
    pub image_height: i16,
    pub image_pict_resource_id: ResourceID,
    /// File path to headerless pict data, relative to build dir
    pub image_pict_data_rel: String,
    pub mask_pict_resource_id: Option<ResourceID>,
    /// File path to headerless pict data, relative to build dir
    pub mask_pict_data_rel: Option<String>,
}

impl MaskedPictAsset {
    fn has_mask(&self) -> bool {
        self.mask_pict_resource_id.is_some() && self.mask_pict_data_rel.is_some()
    }
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
    masked_pict_asset_groups: &Vec<(String, Vec<MaskedPictAsset>)>,
    rgn_asset_groups: &Vec<(String, Vec<RGNAsset>)>,
    ninepatch_assets: &Vec<NinePatchAsset>,
    tsx_assets: &Vec<TSXAsset>,
    tmx_assets: &Vec<TMXAsset>,
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

    for (group_name, group_assets) in masked_pict_asset_groups {
        write!(rez, "/* {group_name} */\n\n")?;
        write!(header, "/* {group_name} */\n\n")?;

        for asset in group_assets {
            let base_name = &asset.base_name;

            let image_constant = format!("asset_{group_name}_{base_name}_image_pict_resource_id")
                .to_case(Case::Camel);
            write!(
                rez,
                "read 'PICT' ({image_constant}, \"{group_name} {base_name}\") \"{path}\";\n",
                path = asset.image_pict_data_rel,
            )?;
            write!(
                header,
                "#define {image_constant} {id}\n",
                id = asset.image_pict_resource_id,
            )?;

            if let (Some(id), Some(path)) =
                (&asset.mask_pict_resource_id, &asset.mask_pict_data_rel)
            {
                let mask_constant = format!("asset_{group_name}_{base_name}_mask_pict_resource_id")
                    .to_case(Case::Camel);
                write!(
                    rez,
                    "read 'PICT' ({mask_constant}, \"{group_name} {base_name}\") \"{path}\";\n",
                )?;
                write!(header, "#define {mask_constant} {id}\n",)?;
            }
        }

        write!(rez, "\n")?;
        write!(header, "\n")?;
    }

    // TODO: all assets should support groups

    for (group, rgn_assets) in rgn_asset_groups {
        for rgn_asset in rgn_assets {
            write!(rez, "/* {group} region lists */\n\n")?;
            write!(header, "/* {group} region lists */\n\n")?;

            write!(rez, "{src}", src = rgn_asset.rez())?;
            write!(header, "{src}", src = rgn_asset.header())?;

            write!(rez, "\n")?;
            write!(header, "\n")?;
        }
    }

    for ninepatch_asset in ninepatch_assets {
        write!(rez, "/* sprite sheet 9-patch lists */\n\n")?;
        write!(header, "/* sprite sheet 9-patch lists */\n\n")?;

        write!(rez, "{src}", src = ninepatch_asset.rez())?;
        write!(header, "{src}", src = ninepatch_asset.header())?;

        write!(rez, "\n")?;
        write!(header, "\n")?;
    }

    for tsx_asset in tsx_assets {
        write!(rez, "/* tilesets */\n\n")?;
        write!(header, "/* tilesets */\n\n")?;

        write!(rez, "{src}", src = tsx_asset.rez())?;
        write!(header, "{src}", src = tsx_asset.header())?;

        write!(rez, "\n")?;
        write!(header, "\n")?;
    }

    for tmx_asset in tmx_assets {
        write!(rez, "/* tilemaps */\n\n")?;
        write!(header, "/* tilemaps */\n\n")?;

        write!(rez, "{src}", src = tmx_asset.rez())?;
        write!(header, "{src}", src = tmx_asset.header())?;

        write!(rez, "\n")?;
        write!(header, "\n")?;
    }

    // TODO: make use of `Resourceful` and generalize this

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
