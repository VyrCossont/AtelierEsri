//! Handle Tiled TMX/TSX files.

use crate::assets::{asset_group_foreach, AssetGroup};
use crate::fsutil::ensure_dir;
use crate::mac::resource::TypedResource;
use crate::mac::OSType;
use crate::mac_assets::{
    png_to_pict, MaskedPictAsset, QDRect, RGNAsset, ResourceID, ResourceIDGenerator, Resourceful,
};
use convert_case::{Case, Casing};
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tiled::{
    FiniteTileLayer, LayerTileData, LayerType, Loader, ObjectLayer, ObjectShape, Orientation,
    TileLayer, Tileset,
};

const TILEMAP_ASSETS: &[AssetGroup] = &[AssetGroup {
    name: "map",
    srcs: &["maps/*.tmx"],
}];

pub fn compile_maps(
    asset_base_dir: &Path,
    build_dir: &Path,
    resource_id_generator: &mut ResourceIDGenerator,
) -> anyhow::Result<(
    Vec<MaskedPictAsset>,
    Vec<TSXAsset>,
    Vec<RGNAsset>,
    Vec<TMXAsset>,
)> {
    // Map from canonicalized absolute path to image asset.
    // Used for de-duping image data used by multiple tilesets.
    let mut tileset_image_assets_by_path = BTreeMap::<PathBuf, MaskedPictAsset>::new();

    // Map from tileset name to tileset asset.
    // Used for de-duping tilesets used by multiple tilemaps.
    // Assumes tileset names are unique, since we lose path info when loading through this crate.
    let mut tileset_assets_by_name = BTreeMap::<String, TSXAsset>::new();

    let mut tilemap_rgn_assets = Vec::<RGNAsset>::new();

    let mut tilemap_assets = Vec::<TMXAsset>::new();

    // While we could have more than one group of maps,
    // tilesets and tileset images may be shared arbitrarily,
    // so they'll all go in one group directory.
    let tileset_group_dir = build_dir.join("tileset");
    ensure_dir(&tileset_group_dir)?;

    let mut loader = Loader::new();

    // Discover all tilemaps.
    let glob_match_fn = |_group_name: &str,
                         _group_dir: &Path,
                         src: &Path,
                         base_name: &OsStr,
                         _ext: &str|
     -> anyhow::Result<()> {
        let tilemap_asset = load_map(
            build_dir,
            resource_id_generator,
            &mut tileset_image_assets_by_path,
            &mut tileset_assets_by_name,
            &mut tilemap_rgn_assets,
            &tileset_group_dir,
            &mut loader,
            src,
            base_name,
        )?;
        tilemap_assets.push(tilemap_asset);
        Ok(())
    };

    // No-op.
    let group_fn = |_group_name: &str, _group_dir: &Path| -> anyhow::Result<()> { Ok(()) };

    asset_group_foreach(
        TILEMAP_ASSETS,
        asset_base_dir,
        build_dir,
        glob_match_fn,
        group_fn,
    )?;

    let tileset_image_assets = tileset_image_assets_by_path.into_values().collect();
    let tileset_assets = tileset_assets_by_name.into_values().collect();

    let tilemap_group_dir = build_dir.join("map");
    ensure_dir(&tilemap_group_dir)?;
    codegen(
        &tileset_assets,
        &tileset_group_dir,
        &tilemap_assets,
        &tilemap_group_dir,
    )?;

    Ok((
        tileset_image_assets,
        tileset_assets,
        tilemap_rgn_assets,
        tilemap_assets,
    ))
}

fn codegen(
    tileset_assets: &Vec<TSXAsset>,
    tileset_group_dir: &Path,
    tilemap_assets: &Vec<TMXAsset>,
    tilemap_group_dir: &Path,
) -> anyhow::Result<()> {
    {
        // Tilesets

        {
            let hpp_path = tileset_group_dir.join("TSXData.hpp");
            let mut hpp = BufWriter::new(
                File::options()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&hpp_path)?,
            );

            write!(hpp, "#pragma once\n")?;
            write!(hpp, "\n")?;
            write!(hpp, "#include <cstdint>\n")?;
            write!(hpp, "#include <optional>\n")?;
            write!(hpp, "\n")?;
            write!(hpp, "#include \"Resource.hpp\"\n")?;
            write!(hpp, "\n")?;
            write!(hpp, "namespace AtelierEsri {{\n")?;
            write!(hpp, "\n")?;

            write!(hpp, "{src}\n", src = TSXAsset::HPP)?;

            for tileset_asset in tileset_assets {
                write!(hpp, "{src}\n", src = tileset_asset.hpp())?;
            }

            write!(hpp, "\n")?;
            write!(hpp, "}}  // namespace AtelierEsri\n")?;
        }

        {
            let cpp_path = tileset_group_dir.join("TSXData.cpp");
            let mut cpp = BufWriter::new(
                File::options()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&cpp_path)?,
            );

            write!(cpp, "#include \"TSXData.hpp\"\n")?;
            write!(cpp, "\n")?;
            write!(cpp, "namespace AtelierEsri {{\n")?;
            write!(cpp, "\n")?;

            for tileset_asset in tileset_assets {
                write!(cpp, "{src}\n", src = tileset_asset.cpp())?;
            }

            write!(cpp, "\n")?;
            write!(cpp, "}}  // namespace AtelierEsri\n")?;
        }

        // Tilemaps

        {
            let hpp_path = tilemap_group_dir.join("TMXData.hpp");
            let mut hpp = BufWriter::new(
                File::options()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&hpp_path)?,
            );

            write!(hpp, "#pragma once\n")?;
            write!(hpp, "\n")?;
            write!(hpp, "#include <cstdint>\n")?;
            write!(hpp, "#include <vector>\n")?;
            write!(hpp, "\n")?;
            write!(hpp, "#include \"Resource.hpp\"\n")?;
            write!(hpp, "\n")?;
            write!(hpp, "namespace AtelierEsri {{\n")?;
            write!(hpp, "\n")?;

            write!(hpp, "{src}\n", src = TMXAsset::HPP)?;

            for tilemap_asset in tilemap_assets {
                write!(hpp, "{src}\n", src = tilemap_asset.hpp())?;
            }

            write!(hpp, "\n")?;
            write!(hpp, "}}  // namespace AtelierEsri\n")?;
        }

        {
            let cpp_path = tilemap_group_dir.join("TMXData.cpp");
            let mut cpp = BufWriter::new(
                File::options()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&cpp_path)?,
            );

            write!(cpp, "#include \"TMXData.hpp\"\n")?;
            write!(cpp, "\n")?;
            write!(cpp, "namespace AtelierEsri {{\n")?;
            write!(cpp, "\n")?;

            for tilemap_asset in tilemap_assets {
                write!(cpp, "{src}\n", src = tilemap_asset.cpp())?;
            }

            write!(cpp, "\n")?;
            write!(cpp, "}}  // namespace AtelierEsri\n")?;
        }
    }

    Ok(())
}

/// Convert tileset image to masked PICT asset, or retrieve it if already converted.
fn get_tileset_image_asset(
    build_dir: &Path,
    resource_id_generator: &mut ResourceIDGenerator,
    tileset_image_assets_by_path: &mut BTreeMap<PathBuf, MaskedPictAsset>,
    tileset_group_dir: &Path,
    tileset: &Arc<Tileset>,
) -> anyhow::Result<MaskedPictAsset> {
    let image_path = tileset
        .image
        .as_ref()
        .ok_or(anyhow::anyhow!(
            "No image for tileset {tileset}",
            tileset = tileset.name
        ))?
        .source
        .clone();

    let canonical_image_path = image_path.canonicalize()?;
    let tileset_image_asset = match tileset_image_assets_by_path.entry(canonical_image_path) {
        Entry::Occupied(entry) => entry.get().clone(),

        Entry::Vacant(entry) => {
            if image_path.extension() != Some(&OsString::from("png")) {
                anyhow::bail!("Only PNG tileset images are supported right now");
            }

            // Copy image to tilesets group directory.
            let tileset_group_dir_image_path = tileset_group_dir.join(
                image_path
                    .file_name()
                    .ok_or(anyhow::anyhow!("Couldn't get file name for tileset image"))?,
            );
            fs::copy(&image_path, &tileset_group_dir_image_path)?;

            let image_base_name = image_path
                .file_stem()
                .ok_or(anyhow::anyhow!("Couldn't get file stem for tileset image"))?
                .to_string_lossy()
                .to_string();

            entry
                .insert(png_to_pict(
                    build_dir,
                    image_base_name,
                    resource_id_generator,
                    &tileset_group_dir_image_path,
                )?)
                .clone()
        }
    };

    Ok(tileset_image_asset)
}

/// Convert tileset image to tileset asset, or retrieve it if already converted.
fn get_tileset_asset(
    build_dir: &Path,
    resource_id_generator: &mut ResourceIDGenerator,
    tileset_image_assets_by_path: &mut BTreeMap<PathBuf, MaskedPictAsset>,
    tileset_assets_by_name: &mut BTreeMap<String, TSXAsset>,
    tileset_group_dir: &Path,
    tileset: &Arc<Tileset>,
) -> anyhow::Result<TSXAsset> {
    let tileset_asset = match tileset_assets_by_name.entry(tileset.name.clone()) {
        Entry::Occupied(entry) => entry.get().clone(),

        Entry::Vacant(entry) => {
            let tileset_image_asset = get_tileset_image_asset(
                build_dir,
                resource_id_generator,
                tileset_image_assets_by_path,
                tileset_group_dir,
                tileset,
            )?;
            entry
                .insert(TSXAsset::new(
                    resource_id_generator,
                    tileset,
                    &tileset_image_asset,
                )?)
                .clone()
        }
    };

    Ok(tileset_asset)
}

pub trait Codegen {
    const HPP: &'static str;
    fn data_id(&self) -> String;
    fn hpp(&self) -> String;
    fn cpp(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct TSXAsset {
    resource_id: ResourceID,
    name: String,
    // Pad to word.
    tile_width: u16,
    tile_height: u16,
    image_width: u16,
    image_height: u16,
    /// ID of a `PICT` resource.
    image_pict_resource_id: ResourceID,
    /// ID of a `PICT` resource.
    /// Optional: can be serialized as ID `0`, which is not usable by app resources.
    mask_pict_resource_id: Option<ResourceID>,
}

impl TSXAsset {
    fn new(
        resource_id_generator: &mut ResourceIDGenerator,
        tileset: &Arc<Tileset>,
        tileset_image_asset: &MaskedPictAsset,
    ) -> anyhow::Result<Self> {
        let MaskedPictAsset {
            image_width,
            image_height,
            image_pict_resource_id,
            mask_pict_resource_id,
            ..
        } = tileset_image_asset;
        Ok(Self {
            resource_id: resource_id_generator.get(Self::OS_TYPE),
            name: tileset.name.clone(),
            tile_width: u16::try_from(tileset.tile_width)?,
            tile_height: u16::try_from(tileset.tile_height)?,
            image_width: *image_width as u16,
            image_height: *image_height as u16,
            image_pict_resource_id: *image_pict_resource_id,
            mask_pict_resource_id: *mask_pict_resource_id,
        })
    }
}

impl TypedResource for TSXAsset {
    const OS_TYPE: OSType = *b"TSX ";
}

impl Resourceful for TSXAsset {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn rez(&self) -> String {
        let mut acc = Vec::<String>::new();

        acc.push(format!(
            "resource '{os_type}' ({id_constant}, \"{name}\") {{",
            os_type = Self::os_type_rez(),
            name = self.name,
            id_constant = self.id_constant(),
        ));

        acc.push(format!(
            "    {tile_width}, {tile_height},",
            tile_width = self.tile_width,
            tile_height = self.tile_height
        ));

        acc.push(format!(
            "    {image_width}, {image_height},",
            image_width = self.image_width,
            image_height = self.image_height
        ));

        acc.push(format!(
            "    {image_pict_resource_id}, {mask_pict_resource_id},",
            image_pict_resource_id = self.image_pict_resource_id,
            mask_pict_resource_id = self.mask_pict_resource_id.unwrap_or(0)
        ));

        acc.push("};\n".to_string());

        acc.join("\n")
    }

    fn header(&self) -> String {
        // TSX assets shouldn't need to be referenced directly, but we do need ID constants for Rez definitions.
        format!(
            "#define {id_constant} {id}",
            id_constant = self.id_constant(),
            id = self.resource_id,
        )
    }
}

impl Codegen for TSXAsset {
    const HPP: &'static str = r#"struct TSXAsset {
  uint16_t tile_width;
  uint16_t tile_height;
  uint16_t image_width;
  uint16_t image_height;
  ResourceID image_pict_resource_id;
  std::optional<ResourceID> mask_pict_resource_id;
};
"#;

    fn data_id(&self) -> String {
        self.id_constant()
            .strip_suffix("ResourceId")
            .expect("Missing ResourceId suffix")
            .to_string()
    }

    fn hpp(&self) -> String {
        format!(r#"extern const TSXAsset {id};"#, id = self.data_id())
    }

    fn cpp(&self) -> String {
        format!(
            r#"const TSXAsset {id}{{
  .tile_width = {tile_width},
  .tile_height = {tile_height},
  .image_width = {image_width},
  .image_height = {image_height},
  .image_pict_resource_id = {image_pict_resource_id},
  .mask_pict_resource_id = {mask_pict_resource_id},
}};
"#,
            id = self.data_id(),
            tile_width = self.tile_width,
            tile_height = self.tile_height,
            image_width = self.image_width,
            image_height = self.image_height,
            image_pict_resource_id = self.image_pict_resource_id,
            mask_pict_resource_id = self
                .mask_pict_resource_id
                .map(|id| id.to_string())
                .unwrap_or("{}".to_string()),
        )
    }
}

/// Tile map.
#[derive(Debug, Clone)]
pub struct TMXAsset {
    /// ID of this `TMX ` resource.
    /// From file; maps don't have internal names.
    name: String,
    resource_id: ResourceID,
    /// In tiles.
    width: u16,
    /// In tiles.
    height: u16,
    /// In pixels.
    tile_width: u16,
    /// In pixels.
    tile_height: u16,
    /// TSX resources.
    tileset_resource_ids: Vec<ResourceID>,
    tile_layers: Vec<TMXTileLayer>,
    region_groups: Vec<TMXRegionGroup>,
}

impl TypedResource for TMXAsset {
    const OS_TYPE: OSType = *b"TMX ";
}

impl Resourceful for TMXAsset {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn rez(&self) -> String {
        let mut acc = Vec::<String>::new();

        acc.push(format!(
            "resource '{os_type}' ({id_constant}, \"{name}\") {{",
            os_type = Self::os_type_rez(),
            name = self.name,
            id_constant = self.id_constant(),
        ));

        // Tileset resource IDs.
        acc.push("    {".to_string());
        for tileset_resource_id in &self.tileset_resource_ids {
            acc.push(format!("        {tileset_resource_id},"));
        }
        acc.push("    },".to_string());

        // Tile layers.
        acc.push("    {".to_string());
        for tile_layer in &self.tile_layers {
            acc.push(format!(
                "        \"{name}\", {width}, {height},",
                name = tile_layer.name,
                width = tile_layer.width,
                height = tile_layer.height
            ));

            acc.push("        {".to_string());
            for tile in &tile_layer.tiles {
                acc.push(format!(
                    "            {flip_h}, {flip_v}, {flip_d}, {tileset_ordinal}, {tile_id},",
                    flip_h = tile.flip_h,
                    flip_v = tile.flip_v,
                    flip_d = tile.flip_d,
                    tileset_ordinal = tile.tileset_ordinal,
                    tile_id = tile.tile_id,
                ));
            }
            acc.push("        },".to_string());
        }
        acc.push("    },".to_string());

        // Region groups.
        acc.push("    {".to_string());
        for region_group in &self.region_groups {
            acc.push(format!(
                "        \"{name}\", {rgn_resource_id},",
                name = region_group.name,
                rgn_resource_id = region_group.rgn_resource_id,
            ));
        }
        acc.push("    },".to_string());

        acc.push("};\n".to_string());

        acc.join("\n")
    }

    fn header(&self) -> String {
        let mut acc = Vec::<String>::new();

        // ID constant for the map itself.
        acc.push(format!(
            "#define {id_constant} {id}",
            id_constant = self.id_constant(),
            id = self.resource_id,
        ));

        // Tile layer indexes.
        acc.push("\n".to_string());
        for (index, tile_layer) in self.tile_layers.iter().enumerate() {
            acc.push(format!(
                "#define {index_constant} {index}",
                index_constant = format!(
                    "asset_{name}_{tile_layer_name}_tile_layer_index",
                    name = self.name(),
                    tile_layer_name = tile_layer.name,
                )
                .to_case(Case::Camel),
            ));
        }

        // Region group indexes.
        acc.push("\n".to_string());
        for (index, region_group) in self.region_groups.iter().enumerate() {
            acc.push(format!(
                "#define {index_constant} {index}",
                index_constant = format!(
                    "asset_{name}_{region_group_name}_region_group_index",
                    name = self.name(),
                    region_group_name = region_group.name,
                )
                .to_case(Case::Camel),
            ));
        }

        acc.join("\n")
    }
}

impl Codegen for TMXAsset {
    const HPP: &'static str = r#"struct TMXTile {
  bool flip_h;
  bool flip_v;
  bool flip_d;
  /// 1 + index into parent map's list of tilesets.
  /// 0 indicates an empty tile position; all other fields should be 0/false.
  uint8_t tileset_ordinal;
  /// ID within tileset.
  uint16_t tile_id;
};

struct TMXTileLayer {
  std::string name;
  /// In tiles.
  uint16_t width;
  /// In tiles.
  uint16_t height;
  std::vector<TMXTile> tiles;
};

struct TMXRegionGroup {
  std::string name;
  ResourceID rgn_resource_id;
};

struct TMXAsset {
  /// In tiles.
  uint16_t width;
  /// In tiles.
  uint16_t height;
  /// In pixels.
  uint16_t tile_width;
  /// In pixels.
  uint16_t tile_height;
  /// 'TSX ' resource IDs.
  std::vector<ResourceID> tileset_resource_ids;
  std::vector<TMXTileLayer> tile_layers;
  std::vector<TMXRegionGroup> region_groups;
};
"#;

    fn data_id(&self) -> String {
        self.id_constant()
            .strip_suffix("ResourceId")
            .expect("Missing ResourceId suffix")
            .to_string()
    }

    fn hpp(&self) -> String {
        format!(r#"extern const TMXAsset {id};"#, id = self.data_id())
    }

    fn cpp(&self) -> String {
        if self.tileset_resource_ids != vec![128] {
            todo!("Handle tileset resolution");
        }
        let tileset_resource_ids: String = {
            self.tileset_resource_ids
                .iter()
                .map(|id| format!("    {id},\n"))
                .collect()
        };

        let tile_layers: String = {
            self.tile_layers
                .iter()
                .map(|tile_layer| {
                    let tiles: String = {
                        tile_layer
                            .tiles
                            .iter()
                            .map(|tile| {
                                format!(
                                    r#"      TMXTile{{
        .flip_h = {flip_h},
        .flip_v = {flip_v},
        .flip_d = {flip_d},
        .tileset_ordinal = {tileset_ordinal},
        .tile_id = {tile_id},
      }},
"#,
                                    flip_h = tile.flip_h,
                                    flip_v = tile.flip_v,
                                    flip_d = tile.flip_d,
                                    tileset_ordinal = tile.tileset_ordinal,
                                    tile_id = tile.tile_id,
                                )
                            })
                            .collect()
                    };

                    format!(
                        r#"    TMXTileLayer{{
      .name = "{name}",
      .width = {width},
      .height = {height},
      .tiles = {{
{tiles}
      }},
    }},
"#,
                        name = tile_layer.name,
                        width = tile_layer.width,
                        height = tile_layer.height,
                    )
                })
                .collect()
        };

        let region_groups: String = {
            self.region_groups
                .iter()
                .map(|region_group| {
                    format!(
                        r#"    TMXRegionGroup{{
      .name = "{name}",
      .rgn_resource_id = {rgn_resource_id},
    }},
"#,
                        name = region_group.name,
                        rgn_resource_id = region_group.rgn_resource_id,
                    )
                })
                .collect()
        };

        format!(
            r#"const TMXAsset {id}{{
  .width = {width},
  .height = {height},
  .tile_width = {tile_width},
  .tile_height = {tile_height},
  .tileset_resource_ids = {{
{tileset_resource_ids}
  }},
  .tile_layers = {{
{tile_layers}
  }},
  .region_groups = {{
{region_groups}
  }},
}};
"#,
            id = self.data_id(),
            width = self.width,
            height = self.height,
            tile_width = self.tile_width,
            tile_height = self.tile_height,
        )
    }
}

/// Tile layer within a map.
#[derive(Debug, Clone)]
struct TMXTileLayer {
    name: String,
    // Pad to word.
    width: u16,
    height: u16,
    tiles: Vec<TMXTile>,
}

/// A single tile position. May be empty.
#[derive(Debug, Clone)]
struct TMXTile {
    flip_h: bool,
    flip_v: bool,
    flip_d: bool,
    /// 1 + index into parent map's list of tilesets.
    /// 0 indicates an empty tile position; all other fields should be 0/false.
    tileset_ordinal: u8,
    /// ID within tileset.
    tile_id: u16,
}

impl TryFrom<&LayerTileData> for TMXTile {
    type Error = anyhow::Error;

    fn try_from(data: &LayerTileData) -> Result<Self, Self::Error> {
        let tileset_index = u8::try_from(data.tileset_index())?;
        let tile_id = u16::try_from(data.id())?;
        Ok(TMXTile {
            flip_h: data.flip_h,
            flip_v: data.flip_v,
            flip_d: data.flip_v,
            tileset_ordinal: 1 + tileset_index,
            tile_id,
        })
    }
}

impl TMXTile {
    fn empty() -> Self {
        Self {
            flip_h: false,
            flip_v: false,
            flip_d: false,
            tileset_ordinal: 0,
            tile_id: 0,
        }
    }
}

/// A named list of map regions, stored in an `RGN#` resource.
#[derive(Debug, Clone)]
struct TMXRegionGroup {
    name: String,
    // Pad to word.
    /// ID of `RGN#` resource.
    rgn_resource_id: ResourceID,
}

fn object_layer_to_tmx_region_group(
    resource_id_generator: &mut ResourceIDGenerator,
    tilemap_rgn_assets: &mut Vec<RGNAsset>,
    map_name: String,
    layer_name: String,
    layer: ObjectLayer,
) -> anyhow::Result<TMXRegionGroup> {
    let mut regions = BTreeMap::<String, QDRect>::new();
    for object in layer.objects() {
        match object.shape {
            ObjectShape::Rect { width, height } => {
                if width < 0.0 || height < 0.0 {
                    anyhow::bail!("Rectangle object's width or height is negative");
                }
                let top = i16_try_from_f32(object.y)?;
                let left = i16_try_from_f32(object.x)?;
                let bottom = top + i16_try_from_f32(height)?;
                let right = top + i16_try_from_f32(width)?;
                regions.insert(
                    object.name.clone(),
                    QDRect {
                        top,
                        left,
                        bottom,
                        right,
                    },
                );
            }
            ObjectShape::Ellipse { .. } => anyhow::bail!("Ellipse objects aren't supported"),
            ObjectShape::Polyline { .. } => anyhow::bail!("Polyline objects aren't supported"),
            ObjectShape::Polygon { .. } => anyhow::bail!("Polygon objects aren't supported"),
            ObjectShape::Point(_, _) => anyhow::bail!("Point objects aren't supported"),
            ObjectShape::Text { .. } => anyhow::bail!("Text objects aren't supported"),
        }
    }

    let rgn_asset = RGNAsset::new(
        resource_id_generator,
        format!("map {map_name} layer {layer_name} rectangular objects"),
        regions,
    );

    let region_group = TMXRegionGroup {
        name: layer_name,
        rgn_resource_id: rgn_asset.resource_id,
    };

    tilemap_rgn_assets.push(rgn_asset);
    Ok(region_group)
}

fn i16_try_from_f32(x: f32) -> anyhow::Result<i16> {
    if x.is_nan()
        || x.is_infinite()
        || x.fract() != 0.0
        || x > i16::MAX as f32
        || x < i16::MIN as f32
    {
        anyhow::bail!("Can't exactly convert f32 {x} to i16");
    }
    Ok(x.trunc() as i16)
}

fn load_map(
    build_dir: &Path,
    resource_id_generator: &mut ResourceIDGenerator,
    tileset_image_assets_by_path: &mut BTreeMap<PathBuf, MaskedPictAsset>,
    tileset_assets_by_name: &mut BTreeMap<String, TSXAsset>,
    tilemap_rgn_assets: &mut Vec<RGNAsset>,
    tileset_group_dir: &Path,
    loader: &mut Loader,
    src: &Path,
    base_name: &OsStr,
) -> anyhow::Result<TMXAsset> {
    let map = loader.load_tmx_map(src)?;
    let name = base_name.to_string_lossy().to_string();

    if map.orientation != Orientation::Orthogonal {
        anyhow::bail!("Only orthogonal rectangular maps are supported");
    }

    let width = u16::try_from(map.width)?;
    let height = u16::try_from(map.height)?;
    let tile_width = u16::try_from(map.tile_width)?;
    let tile_height = u16::try_from(map.tile_height)?;

    let mut tileset_resource_ids = Vec::<ResourceID>::new();
    for tileset in map.tilesets() {
        let tileset_asset = get_tileset_asset(
            build_dir,
            resource_id_generator,
            tileset_image_assets_by_path,
            tileset_assets_by_name,
            tileset_group_dir,
            tileset,
        )?;
        tileset_resource_ids.push(tileset_asset.resource_id);
    }

    let mut tile_layers = Vec::<TMXTileLayer>::new();
    let mut region_groups = Vec::<TMXRegionGroup>::new();
    for layer in map.layers() {
        match layer.layer_type() {
            LayerType::Tiles(tile_layer) => match tile_layer {
                TileLayer::Infinite(_) => {
                    anyhow::bail!("Infinite tile layers aren't supported");
                }
                TileLayer::Finite(finite_tile_layer) => {
                    let tmx_layer =
                        finite_tile_layer_to_tmx_tile_layer(finite_tile_layer, layer.name.clone())?;
                    tile_layers.push(tmx_layer);
                }
            },
            LayerType::Objects(object_layer) => {
                let tmx_region_group = object_layer_to_tmx_region_group(
                    resource_id_generator,
                    tilemap_rgn_assets,
                    name.clone(),
                    layer.name.clone(),
                    object_layer,
                )?;
                region_groups.push(tmx_region_group);
            }
            LayerType::Image(_) => {
                anyhow::bail!("Image layers aren't supported");
            }
            LayerType::Group(_) => {
                anyhow::bail!("Group layers aren't supported");
            }
        }
    }

    let resource_id = resource_id_generator.get(TMXAsset::OS_TYPE);

    Ok(TMXAsset {
        name,
        resource_id,
        width,
        height,
        tile_width,
        tile_height,
        tileset_resource_ids,
        tile_layers,
        region_groups,
    })
}

fn finite_tile_layer_to_tmx_tile_layer(
    layer: FiniteTileLayer,
    name: String,
) -> anyhow::Result<TMXTileLayer> {
    let width = u16::try_from(layer.width())?;
    let height = u16::try_from(layer.height())?;
    let mut tiles = vec![];

    for y in 0..(layer.height() as i32) {
        for x in 0..(layer.width() as i32) {
            let tile = if let Some(data) = layer.get_tile_data(x, y) {
                TMXTile::try_from(data)?
            } else {
                TMXTile::empty()
            };
            tiles.push(tile);
        }
    }

    Ok(TMXTileLayer {
        name,
        width,
        height,
        tiles,
    })
}
