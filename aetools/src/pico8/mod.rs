//! PICO-8 asset format support.

mod custom_character;
mod item_sprite;
mod palette;

use crate::assets::{
    asset_group_foreach, export_or_copy_to_png, CUSTOM_CHAR_ASSET_GROUP, SPRITE_ASSETS,
};
use crate::fsutil::{delete_dir, ensure_dir};
use anyhow::{anyhow, bail, Result};
use custom_character::CustomCharacter;
use glob::glob;
use image::{GenericImage, GrayImage};
use item_sprite::ItemSprite;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

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
        export_or_copy_to_png,
        |group_name: &str, group_dir: &Path| -> Result<()> {
            for glob_result in glob(&group_dir.join("*.png").to_string_lossy())? {
                if let Some(item_sprite) = ItemSprite::load(group_name, &glob_result?)? {
                    item_sprites.push(item_sprite);
                }
            }
            Ok(())
        },
    )?;

    let mut custom_characters = Vec::<CustomCharacter>::new();
    asset_group_foreach(
        vec![&CUSTOM_CHAR_ASSET_GROUP],
        asset_base_dir,
        build_dir,
        export_or_copy_to_png,
        |group_name: &str, group_dir: &Path| -> Result<()> {
            for glob_result in glob(&group_dir.join("*.png").to_string_lossy())? {
                custom_characters.push(CustomCharacter::load(group_name, &glob_result?)?);
            }
            Ok(())
        },
    )?;

    let mut lua_includes = Vec::from(LUA_INCLUDES);

    // Copy Lua files that aren't generated.
    let Some(code_dir) = asset_base_dir.parent().map(|p| p.join("PICO-8")) else {
        bail!("couldn't find Lua source directory");
    };
    for include_path in &lua_includes {
        fs::copy(code_dir.join(include_path), build_dir.join(include_path))?;
    }

    // Contains our custom characters.
    let icons_lua = "icons.lua";
    lua_includes.push(icons_lua);
    let mut icons_writer = BufWriter::new(File::create(build_dir.join(icons_lua))?);
    for cc in custom_characters {
        icons_writer.write(cc.lua_line(false).as_bytes())?;
    }

    // Generate the cartridge itself.
    let mut writer = BufWriter::new(File::create(build_dir.join("atelier_esri.p8"))?);
    writer.write(CARTRIDGE_HEADER)?;
    writer.write(LUA_HEADER)?;
    writer.write(LUA_TITLE_DESC)?;
    for include_path in lua_includes {
        writer.write(b"#include ")?;
        writer.write(include_path.as_bytes())?;
        writer.write(b"\n")?;
    }
    let gfx = Pico8Gfx::from_item_sprites(&item_sprites[..64])?;
    gfx.write(&mut writer)?;

    Ok(())
}

const CARTRIDGE_HEADER: &[u8] = b"\
pico-8 cartridge // http://www.pico-8.com
version 42
";

const LUA_HEADER: &[u8] = b"\
__lua__
";

const LUA_TITLE_DESC: &[u8] = b"\
-- gEOMETRY wARS x aTELIER
-- @vyr@princess.industries

";

/// Order is significant.
const LUA_INCLUDES: &[&str] = &[
    "pinput.lua",
    "input.lua",
    "oop.lua",
    "alchemy.lua",
    "draw.lua",
    "init.lua",
    "aemath.lua",
];

const GFX_HEADER: &[u8] = b"\
__gfx__
";

/// PICO-8 cartridge graphics section.
struct Pico8Gfx {
    /// We'll use this as an indexed color image with the PICO-8 palette.
    image: GrayImage,
}

impl Pico8Gfx {
    fn new(size: Pico8GfxSize) -> Self {
        Self {
            image: GrayImage::new(size.width(), size.height()),
        }
    }

    fn from_item_sprites(item_sprites: &[ItemSprite]) -> Result<Self> {
        // Sprites are all 16x16.
        let required_pixels = 16 * 16 * item_sprites.len() as u32;
        let size = if required_pixels
            <= Pico8GfxSize::Normal.width() * Pico8GfxSize::Normal.height()
        {
            Pico8GfxSize::Normal
        } else if required_pixels <= Pico8GfxSize::Extra.width() * Pico8GfxSize::Extra.height() {
            Pico8GfxSize::Extra
        } else {
            bail!("Too many sprites, won't fit in a PICO-8 gfx section")
        };
        let mut gfx = Self::new(size);
        let mut sx = 0u32;
        let mut sy = 0u32;
        for item_sprite in item_sprites {
            gfx.image.copy_from(&item_sprite.image, sx, sy)?;
            sx += 16;
            if sx > size.width() - 16 {
                sx = 0;
                sy += 16;
            }
        }

        Ok(gfx)
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
            Self::Normal => 64,
            Self::Extra => 128,
        }
    }
}
