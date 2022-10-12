use crate::grey_quantizer::GreyQuantizer;
use crate::palettes;
use aesprite::Unisprite;
use anyhow;
use bitvec::prelude::*;
use image;
use image::{GrayAlphaImage, LumaA};
use std::fs;
use std::path::Path;

fn encode_image(image: &GrayAlphaImage) -> Unisprite<Vec<u8>> {
    let mut quantizer = GreyQuantizer::new();
    for LumaA([l, _]) in image.pixels().cloned() {
        quantizer.count_pixel(l);
    }
    quantizer.reduce(4);
    let (palette, mapping_table) = quantizer.palette_and_mapping_table();
    let palette_map: Vec<u8> = palettes::match_palette_to_target(&palette);

    let num_pixels = (image.width() * image.height()) as usize;
    let mut luma = BitVec::<Msb0, u8>::with_capacity(2 * num_pixels);
    let mut alpha = BitVec::<Msb0, u8>::with_capacity(num_pixels);
    for LumaA([l, a]) in image.pixels().cloned() {
        let l_reduced = mapping_table[l as usize].unwrap();
        let l_target = palette_map[l_reduced as usize];
        luma.extend(&l_target.view_bits::<Msb0>()[u8::BITS as usize - 2..]);
        alpha.push(a == u8::MAX);
    }

    Unisprite {
        w: image.width() as i32,
        h: image.width() as i32,
        luma: luma.into_vec(),
        alpha: alpha.into_vec(),
    }
}

pub fn encode(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    let input_image = image::open(input_path)?.to_luma_alpha8();
    let output_image = encode_image(&input_image);
    let output_src = format!(
        "Unisprite{{ w: {w}, h: {h}, luma: &{luma:#?}, alpha: &{alpha:#?}, }};\n",
        w = output_image.w,
        h = output_image.h,
        luma = output_image.luma,
        alpha = output_image.alpha
    );
    fs::write(output_path, output_src)?;
    Ok(())
}

pub fn decode(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    todo!()
}
