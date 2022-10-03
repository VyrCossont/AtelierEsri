use crate::grey_quantizer::GreyQuantizer;
use crate::palettes;
use aesprite::{Unisprite, UnispriteData, UnispriteMinipalette, WASM4PaletteIndex};
use anyhow;
use bitvec::prelude::*;
use image::{GrayAlphaImage, LumaA};
use std::path::Path;

/*
# expanding 1-bit color to 2-bit color

packed: hgfedcba

p: packed as u16
00000000hgfedcba

## phase 1

q: p << 4
0000_hgfe_dcba_0000

r: p | q
0000_hgfe_xxxx_dcba

s: r & 0b0000_1111_0000_1111
0000_hgfe_0000_dcba

## phase 2

t: s << 2
00_hg_fe_00_00_dc_ba_00

u: s | t
00_hg_xx_fe_00_dc_xx_ba

v: u & 0b00_11_00_11_00_11_00_11
00_hg_00_fe_00_dc_00_ba

# phase 3

w: v << 1
0_h_g_0_0_f_e_0_0_d_c_0_0_b_a_0

x: v | w
0_h_x_g_0_f_x_e_0_d_x_c_0_b_x_a

y: x | 0b0_1_0_1_0_1_0_1_0_1_0_1_0_1_0_1
0_h_0_g_0_f_0_e_0_d_0_c_0_b_0_a

# 1-bit palette

sprite_expanded = y

[0, 1] -> [0, 1]: screen = sprite_expanded
[0, 1] -> [0, 2]: screen = sprite_expanded * 2
[0, 1] -> [0, 3]: screen = sprite_expanded * 3
[0, 1] -> [1, 2]: screen = sprite_expanded + 0b01_01_01_01_01_01_01_01
[0, 1] -> [1, 3]: screen = (sprite_expanded * 2) + 0b01_01_01_01_01_01_01_01
[0, 1] -> [2, 3]: screen = sprite_expanded + 0b10_10_10_10_10_10_10_10
*/

fn has_mask(image: &GrayAlphaImage) -> bool {
    for pixel in image.pixels() {
        let [_, a] = pixel.0;
        if a < u8::MAX {
            return true;
        }
    }
    false
}

struct UnispriteStorage {
    header: Unisprite<'static>,
    planes: Vec<BitVec<Msb0, u8>>,
}

fn convert(image: &GrayAlphaImage) -> Unisprite {
    let has_mask = has_mask(image);

    let mut quantizer = GreyQuantizer::new();
    for LumaA([l, _]) in image.pixels().cloned() {
        quantizer.count_pixel(l);
    }
    quantizer.reduce(4);
    let (palette, mapping_table) = quantizer.palette_and_mapping_table();
    let grey_count = palette.len();
    let palette_map: Vec<WASM4PaletteIndex> = palettes::match_palette_to_target(&palette);

    let data: UnispriteData = if grey_count == 1 {
        let fill: WASM4PaletteIndex = palette_map[0];
        if has_mask {
            UnispriteData::L0A1 { fill, alpha: &[] }
        } else {
            UnispriteData::L0 { fill }
        }
    } else if grey_count == 2 {
        let p0: WASM4PaletteIndex = palette_map[0];
        let p1: WASM4PaletteIndex = palette_map[1];
        let minipalette = match (p0, p1) {
            (0, 1) => UnispriteMinipalette::P01,
            (0, 2) => UnispriteMinipalette::P02,
            (0, 3) => UnispriteMinipalette::P03,
            (1, 2) => UnispriteMinipalette::P12,
            (1, 3) => UnispriteMinipalette::P13,
            (2, 3) => UnispriteMinipalette::P23,
            _ => panic!("Minipalette should not be {:#?}", (p0, p1)),
        };
        if has_mask {
            UnispriteData::L1A1 {
                minipalette,
                indexes: &[],
                alpha: &[],
            }
        } else {
            UnispriteData::L1 {
                minipalette,
                indexes: &[],
            }
        }
    } else {
        if has_mask {
            UnispriteData::L2A1 {
                luma: &[],
                alpha: &[],
            }
        } else {
            UnispriteData::L2 { luma: &[] }
        }
    };

    todo!()
}

pub fn encode(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    todo!()
}

pub fn decode(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    todo!()
}
