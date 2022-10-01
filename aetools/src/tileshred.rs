use crate::grey_quantizer::GreyQuantizer;
use crate::image2bit::{Image2Bit, PixelAccess2Bit, Subimage2Bit};
use crate::palettes;
use crate::palettes::WASM4_COLORS_ALPHA;
use anyhow;
use image::io::Reader as ImageReader;
use image::{GenericImageView, ImageBuffer, LumaA, Rgba, RgbaImage, SubImage};
use std::path::Path;

pub fn convert(
    tile_width: u32,
    tile_height: u32,
    input_path: &Path,
    output_path: &Path,
) -> anyhow::Result<()> {
    let input_img = ImageReader::open(input_path)?.decode()?.into_luma_alpha8();

    let width = input_img.width();
    let height = input_img.height();
    if width % tile_width != 0 || height % tile_height != 0 {
        anyhow::bail!("Image dimensions {width}×{height} are not an integer multiple of tile dimensions {tile_width}×{tile_height}");
    }

    let mut output_img = Image2Bit::new(width, height);
    for y in (0..height).step_by(tile_height as usize) {
        for x in (0..width).step_by(tile_width as usize) {
            let input_tile = input_img.view(x, y, tile_width, tile_height);
            let mut output_tile = output_img.subimage(x, y, tile_width, tile_height);
            recolor(&input_tile, &mut output_tile);
        }
    }

    // Copy alpha from original image and use output image as index into palette.
    let mut rgba_output_img = RgbaImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let LumaA([_, a]) = input_img.get_pixel(x, y);
            *rgba_output_img.get_pixel_mut(x, y) = if *a < u8::MAX {
                Rgba([0x00, 0x00, 0x00, 0x00])
            } else {
                WASM4_COLORS_ALPHA[output_img.get_pixel(x, y) as usize]
            };
        }
    }

    rgba_output_img.save(output_path)?;

    Ok(())
}

fn recolor(
    input_tile: &SubImage<&ImageBuffer<LumaA<u8>, Vec<u8>>>,
    output_tile: &mut Subimage2Bit,
) {
    let mut quantizer = GreyQuantizer::new();
    for (_, _, LumaA([l, a])) in input_tile.pixels() {
        if a < u8::MAX {
            // Treat all transparency as full transparency.
            continue;
        }
        quantizer.count_pixel(l);
    }
    quantizer.reduce(4);
    let (palette, mut table) = quantizer.palette_and_mapping_table();
    let palette_map = palettes::match_palette_to_target(&palette);
    for entry in &mut table {
        *entry = entry.map(|i| palette_map[i as usize]);
    }

    for (x, y, LumaA([l, a])) in input_tile.pixels() {
        if a < u8::MAX {
            continue;
        }
        if let Some(i) = table[l as usize] {
            output_tile.set_pixel(x, y, i);
        }
    }
}
