use anyhow;
use std::path::Path;

use crate::grey_quantizer::GreyQuantizer;
use image::io::Reader as ImageReader;
use image::Luma;

pub fn convert(
    input_path: &Path,
    tile_width: u32,
    tile_height: u32,
    output_path: &Path,
) -> anyhow::Result<()> {
    let mut img = ImageReader::open(input_path)?.decode()?.into_luma8();
    let mut quantizer = GreyQuantizer::new();
    for Luma([c]) in img.pixels().cloned() {
        quantizer.count_pixel(c);
    }
    quantizer.reduce(4);
    let (palette, table) = quantizer.palette_and_mapping_table();
    for (i, c) in palette.into_iter().enumerate() {
        println!("{i:>3}: #{c:02x}{c:02x}{c:02x}");
    }
    Ok(())
}
