use anyhow;
use std::path::Path;

use image::io::Reader as ImageReader;

pub fn convert(
    input_path: &Path,
    tile_width: u32,
    tile_height: u32,
    output_path: &Path,
) -> anyhow::Result<()> {
    let input_img = ImageReader::open(input_path)?.decode()?;
    todo!()
}
