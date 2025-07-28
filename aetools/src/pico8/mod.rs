//! PICO-8 asset format support.

mod palette;

use crate::assets::{asset_group_foreach, SPRITE_ASSETS};
use crate::ext::aseprite;
use crate::fsutil::{delete_dir, ensure_dir};
use crate::pico8::Pico8GfxSize::{Extra, Normal};
use anyhow::{anyhow, bail, Result};
use glob::glob;
use image::{GenericImage, GrayImage};
use png::ColorType;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

struct ItemSprite {
    name: String,
    /// Indexed-color image in PICO-8 palette.
    image: GrayImage,
}

pub fn generate_assets(asset_base_dir: &Path, build_dir: &Path) -> Result<()> {
    delete_dir(build_dir)?;
    ensure_dir(build_dir)?;

    let item_asset_group = SPRITE_ASSETS
        .iter()
        .find(|x| x.name == "item")
        .ok_or(anyhow!("couldn't find item asset group"))?;

    let mut item_sprites = Vec::<ItemSprite>::new();

    asset_group_foreach(
        vec![item_asset_group],
        asset_base_dir,
        build_dir,
        |group_name: &str,
         group_dir: &Path,
         src: &Path,
         base_name: &OsStr,
         ext: &str|
         -> Result<()> {
            match ext {
                "aseprite" => {
                    // Export sprite slices from each Aseprite project into the group directory.
                    aseprite::export_slices(&src, &group_dir)?;
                }
                "png" => {
                    // Copy PNG sprites into the group directory.
                    let mut image_png = group_dir.join(base_name);
                    image_png.set_extension("png");
                    fs::copy(src, image_png)?;
                }
                _ => bail!("Unsupported file extension: {ext}"),
            }
            Ok(())
        },
        |group_name: &str, group_dir: &Path| -> Result<()> {
            for glob_result in glob(&group_dir.join("*.png").to_string_lossy())? {
                let png_slice = glob_result?;

                let base_name = png_slice
                    .file_stem()
                    .ok_or(anyhow::anyhow!("Couldn't get file stem for PNG slice"))?
                    .to_string_lossy()
                    .to_string();

                // Skip border tiles in item asset group.
                if base_name.starts_with("border") {
                    fs::remove_file(png_slice)?;
                    continue;
                };

                let decoder = png::Decoder::new(File::open(&png_slice)?);
                let mut reader = decoder.read_info()?;

                let info = reader.info();
                if info.width != 16
                    || info.height != 16
                    || info.color_type != ColorType::Indexed
                    || info.trns.as_ref().map(|x| x.len()).unwrap_or(0) != 5
                    || info.palette.as_ref().map(|x| x.len()).unwrap_or(0) != 5 * 3
                {
                    fs::remove_file(png_slice)?;
                    continue;
                }
                let palette = four_plus_trans_palette(info)?;

                let mut buf = vec![0; reader.output_buffer_size()];
                let frame_info = reader.next_frame(&mut buf)?;
                let bytes = &buf[..frame_info.buffer_size()];
                if bytes.len() != 16 * 16 {
                    bail!("image is wrong size or we don't know how the PNG library works");
                }
                let mut mapped_bytes = vec![0; 16 * 16];
                for i in 0..bytes.len() {
                    mapped_bytes[i] = palette[bytes[i] as usize];
                }

                let sprite_name = format!("{group_name}_{base_name}");
                item_sprites.push(ItemSprite {
                    name: sprite_name,
                    image: GrayImage::from_raw(16, 16, mapped_bytes).ok_or(anyhow!(
                        "requested image size is bigger than supplied buffer"
                    ))?,
                });
            }
            Ok(())
        },
    )?;

    let mut writer = BufWriter::new(File::create(build_dir.join("craft.p8"))?);
    writer.write(CARTRIDGE_HEADER)?;
    let gfx = gfx_from_item_sprites(&item_sprites[..64])?;
    gfx.write(&mut writer)?;

    Ok(())
}

/// Take a PNG palette that is four shades plus a transparent color,
/// and return a palette that maps that PNG's color indexes to PICO-8's default palette,
/// with black as the transparent color and a suitable ramp for the rest.
fn four_plus_trans_palette(info: &png::Info) -> Result<Vec<u8>> {
    let trns = info
        .trns
        .as_ref()
        .ok_or(anyhow!("should have been checked in group_fn"))?;
    let plte = info
        .palette
        .as_ref()
        .ok_or(anyhow!("should have been checked in group_fn"))?;

    let n = 5;
    let mut palette = vec![0; n];
    let p8_transparent_color = 0;
    // Uses default palette. One isn't actually gray but whatever.
    let p8_gray_ramp = [5, 13, 6, 7];

    let mut regular_colors = Vec::<u8>::with_capacity(n);
    for png_palette_index in 0..n {
        if trns[png_palette_index] == 0 {
            palette[png_palette_index] = p8_transparent_color;
            continue;
        } else {
            regular_colors.push(png_palette_index as u8);
        }
    }
    if regular_colors.len() != 4 {
        bail!(
            "Expected 4 regular colors and 1 transparent, got {} and {}",
            regular_colors.len(),
            n - regular_colors.len()
        )
    }
    regular_colors.sort_by(|a, b| {
        let ai = 3 * *a as usize;
        let ar = plte[ai] as f64;
        let ag = plte[ai + 1] as f64;
        let ab = plte[ai + 2] as f64;
        let alum = (0.33 * ar + 0.5 * ag + 0.16 * ab) as u32;
        let bi = 3 * *b as usize;
        let br = plte[bi] as f64;
        let bg = plte[bi + 1] as f64;
        let bb = plte[bi + 2] as f64;
        let blum = (0.33 * br + 0.5 * bg + 0.16 * bb) as u32;
        alum.cmp(&blum)
    });
    for (p8_gray_ramp_index, png_palette_index) in regular_colors.iter().enumerate() {
        palette[*png_palette_index as usize] = p8_gray_ramp[p8_gray_ramp_index];
    }

    Ok(palette)
}

fn gfx_from_item_sprites(item_sprites: &[ItemSprite]) -> Result<Pico8Gfx> {
    // Sprites are all 16x16.
    let required_pixels = 16 * 16 * item_sprites.len() as u32;
    let size = if required_pixels <= Normal.width() * Normal.height() {
        Normal
    } else if required_pixels <= Extra.width() * Extra.height() {
        Extra
    } else {
        bail!("Too many sprites, won't fit in a PICO-8 gfx section")
    };
    let mut gfx = Pico8Gfx::new(size);
    let mut sx = 0u32;
    let mut sy = 0u32;
    for item_sprite in item_sprites {
        println!("copying {}", item_sprite.name);
        gfx.image.copy_from(&item_sprite.image, sx, sy)?;
        sx += 16;
        if sx > size.width() - 16 {
            sx = 0;
            sy += 16;
        }
    }

    Ok(gfx)
}

/// PICO-8 cartridge graphics section.
struct Pico8Gfx {
    /// We'll use this as an indexed color image with the PICO-8 palette.
    image: GrayImage,
}

#[derive(Copy, Clone)]
enum Pico8GfxSize {
    /// 128x64
    Normal,
    /// 128x128 (overlaps with lower half of map)
    Extra,
}

impl Pico8GfxSize {
    fn width(&self) -> u32 {
        128
    }

    fn height(&self) -> u32 {
        match &self {
            Normal => 64,
            Extra => 128,
        }
    }
}

const CARTRIDGE_HEADER: &[u8] = b"\
pico-8 cartridge // http://www.pico-8.com
version 42
";

const GFX_HEADER: &[u8] = b"\
__gfx__
";

impl Pico8Gfx {
    fn new(size: Pico8GfxSize) -> Self {
        Self {
            image: GrayImage::new(size.width(), size.height()),
        }
    }

    /// Write a gfx section.
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        writer.write(GFX_HEADER)?;
        let mut line_buf = vec![0u8; self.image.width() as usize + 1];
        line_buf[self.image.width() as usize] = b'\n';
        for y in 0..self.image.height() {
            for x in 0..self.image.width() {
                let p = self.image.get_pixel(x, y).0[0];
                match p {
                    0x0..=0x9 => line_buf[x as usize] = b'0' + p,
                    0xa..=0xf => line_buf[x as usize] = b'a' + p - 0xa,
                    _ => bail!("Illegal color index: {p}"),
                }
            }
            writer.write(&line_buf)?;
        }
        Ok(())
    }
}
