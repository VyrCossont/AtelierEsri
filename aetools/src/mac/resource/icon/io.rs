use crate::histogram::DynamicHistogramError;
use bitvec::prelude::*;
use image::{DynamicImage, GrayImage, ImageBuffer, Rgb};

/// Generic conversion to and from images.
pub trait IconIO: Sized {
    /// Color or grayscale image.
    /// Not present if the icon is only a mask, although we haven't implemented those yet.
    fn image(&self) -> Option<DynamicImage>;
    /// Mask converted to alpha as a grayscale image.
    fn mask(&self) -> Option<DynamicImage>;
    /// Try to create an icon from an input image.
    /// Fails if it's the wrong size, colors, etc.
    fn try_from(image: DynamicImage) -> Result<Self, IconIOError>;
}

#[derive(Debug)]
pub enum IconIOError {
    /// Dimensions not appropriate for this type of icon.
    Size,
    /// Color depth not appropriate for this type of icon.
    Depth,
    /// Color palette not appropriate for this type of icon.
    Palette,
    /// Alpha channel not appropriate for mask of this type of icon.
    Alpha,
    /// Error calculating histogram of the input image.
    Histogram(DynamicHistogramError),
}

/// Icon type with a fixed size.
pub trait SizedIcon {
    const ICON_SIZE: (u32, u32);
    const ICON_SIZE_LARGE: (u32, u32) = (32, 32);
    const ICON_SIZE_SMALL: (u32, u32) = (16, 16);
    const ICON_SIZE_MINI: (u32, u32) = (16, 12);
}

/// Expand an indexed color image to 16-bit-per-channel RGB.
/// Yes, QuickDraw palettes are 16 bits per channel.
pub fn apply_quickdraw_palette<const NUM_COLORS: usize>(
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
pub fn expand_grays<const BIT_DEPTH: usize>(
    size: (u32, u32),
    data: &[u8],
    invert: bool,
) -> GrayImage {
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
