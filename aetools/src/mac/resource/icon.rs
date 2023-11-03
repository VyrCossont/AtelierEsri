//! Icon resources.
//! See:
//! - https://preterhuman.net/macstuff/insidemac/Toolbox/Toolbox-448.html
//! - https://preterhuman.net/macstuff/insidemac/Toolbox/Toolbox-101.html#HEADING101-48
//! - https://preterhuman.net/macstuff/insidemac/MoreToolbox/MoreToolbox-269.html

use crate::mac::palette::DEFAULT_8_BIT_COLOR_PALETTE;
use crate::mac::resource::TypedResource;
use crate::mac::OSType;
use binrw::binrw;
use bitvec::prelude::*;
use image::{DynamicImage, GrayImage, ImageBuffer, Rgb};

pub trait IconIO {
    /// Icon dimensions. Fixed per type for everything except `cicn`.
    fn size(&self) -> (u32, u32);
    /// Color or grayscale image.
    /// Not present if the icon is only a mask, although we haven't implemented those yet.
    fn image(&self) -> Option<DynamicImage>;
    /// Mask converted to alpha as a grayscale image.
    fn mask(&self) -> Option<DynamicImage>;
}

/// Expand an indexed color image to 16-bit-per-channel RGB.
/// Yes, QuickDraw palettes are 16 bits per channel.
fn apply_quickdraw_palette<const NUM_COLORS: usize>(
    size: (u32, u32),
    data: &[u8],
    palette: &[[u16; 3]; NUM_COLORS],
) -> ImageBuffer<Rgb<u16>, Vec<u16>> {
    let bit_depth = NUM_COLORS.ilog2() as usize;
    assert!(bit_depth.is_power_of_two());

    let (w, h) = size;
    let mut image = ImageBuffer::<Rgb<u16>, Vec<u16>>::new(w, h);
    let bits = data.view_bits::<Msb0>();
    assert_eq!(
        (w * h) as usize * bit_depth,
        bits.len(),
        "Data length doesn't match image size"
    );

    for (bits, pixel) in bits.chunks_exact(bit_depth).zip(image.pixels_mut()) {
        let color_index: usize = bits.load_be();
        pixel.0 = palette[color_index];
    }
    image
}

/// Expand a grayscale image to 8-bit depth.
/// Note that Mac black & white images are generally upside-down: 0 is *white*.
fn expand_grays<const BIT_DEPTH: usize>(size: (u32, u32), data: &[u8], invert: bool) -> GrayImage {
    assert!(BIT_DEPTH.is_power_of_two());
    assert!(BIT_DEPTH <= u8::BITS as usize);

    let (w, h) = size;
    let mut image = GrayImage::new(w, h);

    let bits = data.view_bits::<Msb0>();
    assert_eq!(
        (w * h) as usize * BIT_DEPTH,
        bits.len(),
        "Data length doesn't match image size"
    );

    for (bits, pixel) in bits.chunks_exact(BIT_DEPTH).zip(image.pixels_mut()) {
        // Repeat bits of value until it fills the byte, thus scaling it to the range of a byte.
        let value: u8 = bits.load_be();
        let mut luma: u8 = 0;
        for _ in 0..(u8::BITS / BIT_DEPTH as u32) {
            luma <<= BIT_DEPTH;
            luma |= value;
        }
        if invert {
            luma = !luma;
        }
        pixel.0 = [luma];
    }
    image
}

/// System 1 and up.
/// Can be used in `DITL` and `MENU` resources.
#[binrw]
#[brw(big)]
pub struct Icon1BitLargeOldest {
    pub image_data: [u8; 128],
}

impl TypedResource for Icon1BitLargeOldest {
    const OS_TYPE: OSType = *b"ICON";
}

impl IconIO for Icon1BitLargeOldest {
    fn size(&self) -> (u32, u32) {
        (32, 32)
    }

    fn image(&self) -> Option<DynamicImage> {
        Some(DynamicImage::ImageLuma8(expand_grays::<1>(
            self.size(),
            &self.image_data,
            true,
        )))
    }

    fn mask(&self) -> Option<DynamicImage> {
        None
    }
}

/// Probably System 1 and up.
/// Can be used in `MENU` resources.
/// Normally used as an icon and mask pair, but technically a list.
/// Mask may be omitted; there is no unmasked equivalent.
#[binrw]
#[brw(big)]
#[br(import(data_len: usize))]
pub struct Icon1BitSmallMaskedOldest {
    #[br(count = data_len / 32)]
    pub image_datas: Vec<[u8; 32]>,
}

impl TypedResource for Icon1BitSmallMaskedOldest {
    const OS_TYPE: OSType = *b"SICN";
}

impl IconIO for Icon1BitSmallMaskedOldest {
    fn size(&self) -> (u32, u32) {
        (16, 16)
    }

    fn image(&self) -> Option<DynamicImage> {
        if self.image_datas.len() < 1 {
            return None;
        }

        Some(DynamicImage::ImageLuma8(expand_grays::<1>(
            self.size(),
            &self.image_datas[0],
            true,
        )))
    }

    fn mask(&self) -> Option<DynamicImage> {
        if self.image_datas.len() < 2 {
            return None;
        }

        Some(DynamicImage::ImageLuma8(expand_grays::<1>(
            self.size(),
            &self.image_datas[1],
            true,
        )))
    }
}

/// Color QuickDraw icon, probably System 6 and up.
/// Can be used in `DITL` and `MENU` resources as a counterpart to an `ICON` or `SICN`.
/// Contains both bitmap and pixmap versions, with mask and color table.
/// Data can be packed.
#[binrw]
#[brw(big)]
#[br(import(data_len: usize))]
pub struct IconColorMaskedOldest {
    #[br(count = data_len)]
    pub data: Vec<u8>,
}

impl TypedResource for IconColorMaskedOldest {
    const OS_TYPE: OSType = *b"cicn";
}

/// Normally used as an icon and mask pair, but technically a list.
/// Can be part of an icon suite.
/// Mask may be omitted, but won't be if it's part of an icon suite.
/// System 6 and up.
#[binrw]
#[brw(big)]
#[br(import(data_len: usize))]
pub struct Icon1BitLargeMasked {
    #[br(count = data_len / 128)]
    pub image_datas: Vec<[u8; 128]>,
}

impl TypedResource for Icon1BitLargeMasked {
    const OS_TYPE: OSType = *b"ICN#";
}

/// Can be part of an icon suite.
/// System 6 and up.
#[binrw]
#[brw(big)]
pub struct Icon1BitSmallMasked {
    pub image_data: [u8; 32],
    pub mask_data: [u8; 32],
}

impl TypedResource for Icon1BitSmallMasked {
    const OS_TYPE: OSType = *b"ics#";
}

/// System 6 and up.
/// Can be part of an icon suite.
/// Not supported by ResEdit.
#[binrw]
#[brw(big)]
pub struct Icon1BitMiniMasked {
    pub image_data: [u8; 24],
    pub mask_data: [u8; 24],
}

impl TypedResource for Icon1BitMiniMasked {
    const OS_TYPE: OSType = *b"icm#";
}

/// System 7 and up.
/// Can be part of an icon suite.
#[binrw]
#[brw(big)]
pub struct Icon4BitLarge {
    pub image_data: [u8; 512],
}

impl TypedResource for Icon4BitLarge {
    const OS_TYPE: OSType = *b"icl4";
}

/// System 7 and up.
/// Can be part of an icon suite.
#[binrw]
#[brw(big)]
pub struct Icon4BitSmall {
    pub image_data: [u8; 128],
}

impl TypedResource for Icon4BitSmall {
    const OS_TYPE: OSType = *b"ics4";
}

/// System 7 and up.
/// Can be part of an icon suite.
/// Not supported by ResEdit.
#[binrw]
#[brw(big)]
pub struct Icon4BitMini {
    pub image_data: [u8; 96],
}

impl TypedResource for Icon4BitMini {
    const OS_TYPE: OSType = *b"icm4";
}

/// System 7 and up.
/// Can be part of an icon suite.
#[binrw]
#[brw(big)]
pub struct Icon8BitLarge {
    pub image_data: [u8; 1024],
}

impl TypedResource for Icon8BitLarge {
    const OS_TYPE: OSType = *b"icl8";
}

/// System 7 and up.
/// Can be part of an icon suite.
#[binrw]
#[brw(big)]
pub struct Icon8BitSmall {
    pub image_data: [u8; 256],
}

impl TypedResource for Icon8BitSmall {
    const OS_TYPE: OSType = *b"ics8";
}

impl IconIO for Icon8BitSmall {
    fn size(&self) -> (u32, u32) {
        (16, 16)
    }

    fn image(&self) -> Option<DynamicImage> {
        Some(DynamicImage::ImageRgb16(apply_quickdraw_palette(
            self.size(),
            &self.image_data,
            &DEFAULT_8_BIT_COLOR_PALETTE,
        )))
    }

    fn mask(&self) -> Option<DynamicImage> {
        None
    }
}

/// System 7 and up.
/// Can be part of an icon suite.
/// Not supported by ResEdit.
#[binrw]
#[brw(big)]
pub struct Icon8BitMini {
    pub image_data: [u8; 192],
}

impl TypedResource for Icon8BitMini {
    const OS_TYPE: OSType = *b"icm8";
}
