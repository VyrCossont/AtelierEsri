use anyhow::{anyhow, bail};
use image::GrayImage;
use png::ColorType;
use std::fs;
use std::fs::File;
use std::path::Path;

pub struct ItemSprite {
    pub name: String,
    /// Indexed-color image in PICO-8 palette.
    pub image: GrayImage,
}

impl ItemSprite {
    /// Load a 4-color + 1-transparent-color item-sized PNG into an indexed-color image
    /// using a PICO-8 default palette gray ramp.
    pub fn load(group_name: &str, png_path: &Path) -> anyhow::Result<Option<Self>> {
        let base_name = png_path
            .file_stem()
            .ok_or(anyhow!("Couldn't get file stem for PNG path"))?
            .to_string_lossy()
            .to_string();

        // Skip border tiles in item asset group.
        if base_name.starts_with("border") {
            fs::remove_file(png_path)?;
            return Ok(None);
        };

        let decoder = png::Decoder::new(File::open(&png_path)?);
        let mut reader = decoder.read_info()?;

        // Skip any item sprites that aren't the size we expect.
        let info = reader.info();
        if info.width != 16
            || info.height != 16
            || info.color_type != ColorType::Indexed
            || info.trns.as_ref().map(|x| x.len()).unwrap_or(0) != 5
            || info.palette.as_ref().map(|x| x.len()).unwrap_or(0) != 5 * 3
        {
            fs::remove_file(png_path)?;
            return Ok(None);
        }
        let palette = Self::four_plus_trans_palette(info)?;

        // Read the first frame of the PNG. There should only be one.
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
        Ok(Some(ItemSprite {
            name: sprite_name,
            image: GrayImage::from_raw(16, 16, mapped_bytes).ok_or(anyhow!(
                "requested image size is bigger than supplied buffer"
            ))?,
        }))
    }

    /// Take a PNG palette that is four shades plus a transparent color,
    /// and return a palette that maps that PNG's color indexes to PICO-8's default palette,
    /// with black as the transparent color and a suitable ramp for the rest.
    fn four_plus_trans_palette(info: &png::Info) -> anyhow::Result<Vec<u8>> {
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
        // Sort PNG colors by approx perceived luminosity.
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
        // Map the sorted PNG colors to the gray ramp.
        for (p8_gray_ramp_index, png_palette_index) in regular_colors.iter().enumerate() {
            palette[*png_palette_index as usize] = p8_gray_ramp[p8_gray_ramp_index];
        }

        Ok(palette)
    }
}
