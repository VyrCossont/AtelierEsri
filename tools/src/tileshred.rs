use anyhow;
use std::path::Path;

use crate::grey_quantizer::GreyQuantizer;
use crate::image2bit::{Image2Bit, PixelAccess2Bit, Subimage2Bit};
use image::io::Reader as ImageReader;
use image::{GenericImageView, ImageBuffer, LumaA, Rgba, RgbaImage, SubImage};

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
    let (palette, mut table) = quantizer.palette_and_mapping_table();
    for (i, c) in palette.iter().enumerate() {
        println!("{i:>3}: #{c:02x}{c:02x}{c:02x}");
    }
    println!();

    // Replace the replacement colors with best-match WASM-4 palette indexes.
    // if palette.len() == 4 {
    // Palette length is an exact match.
    // Map the darkest grey to the darkest WASM-4 color, etc., 1:1.
    // }
    for replacement_color in table.iter_mut() {
        let palette_index = palette.binary_search(replacement_color).unwrap_or(0) as u8;
        *replacement_color = palette_index;
    }
    // }

    // let wasm4_greys: [u8; 4] = [0x42, 0x7c, 0xb2, 0xdc];

    for (x, y, LumaA([l, a])) in input_tile.pixels() {
        if a < u8::MAX {
            continue;
        }
        output_tile.set_pixel(x, y, table[l as usize]);
    }
}
