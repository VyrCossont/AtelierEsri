use anyhow;
use std::path::Path;

use crate::grey_quantizer::GreyQuantizer;
use crate::image2bit::{Image2Bit, PixelAccess2Bit, Subimage2Bit};
use image::io::Reader as ImageReader;
use image::{GenericImageView, ImageBuffer, LumaA, Rgba, RgbaImage, SubImage};
use itertools::Itertools;

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

    // Default WASM-4 palette.
    let wasm4_colors: [Rgba<u8>; 4] = [
        Rgba([0x07, 0x18, 0x21, 0xff]),
        Rgba([0x30, 0x68, 0x50, 0xff]),
        Rgba([0x86, 0xc0, 0x6c, 0xff]),
        Rgba([0xe0, 0xf8, 0xcf, 0xff]),
    ];

    // Copy alpha from original image and use output image as index into palette.
    let mut rgba_output_img = RgbaImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let LumaA([_, a]) = input_img.get_pixel(x, y);
            *rgba_output_img.get_pixel_mut(x, y) = if *a < u8::MAX {
                Rgba([0x00, 0x00, 0x00, 0x00])
            } else {
                wasm4_colors[output_img.get_pixel(x, y) as usize]
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
    let (mut palette, mut table) = quantizer.palette_and_mapping_table();
    for (i, c) in palette.iter().enumerate() {
        println!("{i:>3}: #{c:02x}{c:02x}{c:02x}");
    }
    println!();

    // If the palette length is an exact match to the target palette length, we map the darkest grey to the darkest target color, etc., 1:1.
    // Otherwise, find the best fit between the palette and a target palette.
    // For example, if the input image uses only bright greys,
    // we want to map those lumas to bright colors in the target palette.
    if palette.len() < 4 {
        // Default WASM-4 palette converted to greyscale.
        let wasm4_greys: [u8; 4] = [0x42, 0x7c, 0xb2, 0xdc];
        let mut best_squared_error: Option<i32> = None;
        let mut best_palette: Option<Vec<u8>> = None;
        for candidate_palette_indexes in [0usize, 1, 2, 3].into_iter().combinations(palette.len()) {
            let mut candidate_palette: [Option<u8>; 4] = [None; 4];
            // Insert the quantizer-generated palette in order into
            // these indexes on a 4-entry palette.
            let mut p = 0;
            for i in candidate_palette_indexes {
                candidate_palette[i] = Some(palette[p]);
                p += 1;
            }
            // Compare the palette to the target palette,
            // looking only at entries that the image will use.
            let squared_error: i32 = wasm4_greys
                .iter()
                .cloned()
                .zip(candidate_palette)
                .filter_map(|(w, maybe_c)| maybe_c.map(|c| c as i32 - w as i32))
                .map(|e| e * e)
                .sum();
            println!("{squared_error} {candidate_palette:?}");
            if best_squared_error.is_none() || squared_error < best_squared_error.unwrap() {
                best_squared_error = Some(squared_error);
                // We can fill in the unused entries with 0
                // since the input image won't use them.
                best_palette = Some(
                    candidate_palette
                        .into_iter()
                        .map(|maybe_c| maybe_c.unwrap_or(0))
                        .collect::<Vec<_>>(),
                );
            }
        }
        palette = best_palette.unwrap();
    }
    for replacement_color in table.iter_mut() {
        let palette_index = palette.binary_search(replacement_color).unwrap_or(0) as u8;
        *replacement_color = palette_index;
    }

    for (x, y, LumaA([l, a])) in input_tile.pixels() {
        if a < u8::MAX {
            continue;
        }
        output_tile.set_pixel(x, y, table[l as usize]);
    }
}
