use anyhow;
use image::GrayAlphaImage;
use std::path::Path;

/// Allowed cases:
/// - 1 grey, 1 alpha: this is just a rectangle really
/// - 2 grey, 1 alpha: 1-bit grey, no mask
/// - 3 or 4 grey, 1 alpha: 2-bit grey, no mask
/// - 1 or 2 grey, 2 alphas: 1-bit grey, 1-bit mask
///
/// # expanding 1-bit color to 2-bit color
///
/// packed: hgfedcba
///
/// p: packed as u16
/// 00000000hgfedcba
///
/// ## phase 1
///
/// q: p << 4
/// 0000_hgfe_dcba_0000
///
/// r: p | q
/// 0000_hgfe_xxxx_dcba
///
/// s: r & 0b0000_1111_0000_1111
/// 0000_hgfe_0000_dcba
///
/// ## phase 2
///
/// t: s << 2
/// 00_hg_fe_00_00_dc_ba_00
///
/// u: s | t
/// 00_hg_xx_fe_00_dc_xx_ba
///
/// v: u & 0b00_11_00_11_00_11_00_11
/// 00_hg_00_fe_00_dc_00_ba
///
/// # phase 3
///
/// w: v << 1
/// 0_h_g_0_0_f_e_0_0_d_c_0_0_b_a_0
///
/// x: v | w
/// 0_h_x_g_0_f_x_e_0_d_x_c_0_b_x_a
///
/// y: x | 0b0_1_0_1_0_1_0_1_0_1_0_1_0_1_0_1
/// 0_h_0_g_0_f_0_e_0_d_0_c_0_b_0_a
///
/// # 1-bit palette
///
/// sprite_expanded = y
///
/// [0, 1] -> [0, 1]: screen = sprite_expanded
/// [0, 1] -> [0, 2]: screen = sprite_expanded * 2
/// [0, 1] -> [0, 3]: screen = sprite_expanded * 3
/// [0, 1] -> [1, 2]: screen = sprite_expanded + 0b01_01_01_01_01_01_01_01
/// [0, 1] -> [1, 3]: screen = (sprite_expanded * 2) + 0b01_01_01_01_01_01_01_01
/// [0, 1] -> [2, 3]: screen = sprite_expanded + 0b10_10_10_10_10_10_10_10

enum ColorMode {
    L0,
    L0A1,
    L1,
    L1A1,
    L2,
    L2A1,
}

fn detect_color_mode(image: &GrayAlphaImage) -> ColorMode {
    let mut greys = [false; u8::MAX as usize];
    let mut grey_count = 0;
    let mut has_mask = false;
    for pixel in image.pixels() {
        let [l, a] = pixel.0;
        if a < u8::MAX {
            has_mask = true;
        } else {
            if !greys[l as usize] {
                grey_count += 1;
            }
            greys[l as usize] = true;
        }
    }

    if grey_count <= 1 {
        if has_mask {
            ColorMode::L0A1
        } else {
            ColorMode::L0
        }
    } else if grey_count <= 2 {
        if has_mask {
            ColorMode::L1A1
        } else {
            ColorMode::L1
        }
    } else {
        if has_mask {
            ColorMode::L2A1
        } else {
            ColorMode::L2
        }
    }
}

pub fn encode(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    todo!()
}

pub fn decode(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    todo!()
}
