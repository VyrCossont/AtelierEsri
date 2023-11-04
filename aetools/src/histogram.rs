use image::{DynamicImage, ImageBuffer, Luma, LumaA, Pixel, Primitive, Rgb, Rgba};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;

/// Histogram type corresponding to `image::DynamicImage`.
/// Separate bins for color and alpha (if present).
#[derive(Debug, Clone)]
pub enum DynamicHistogram {
    HistogramLuma8 {
        colors: HashMap<Luma<u8>, usize>,
    },
    HistogramLumaA8 {
        colors: HashMap<Luma<u8>, usize>,
        alphas: HashMap<u8, usize>,
    },
    HistogramRgb8 {
        colors: HashMap<Rgb<u8>, usize>,
    },
    HistogramRgba8 {
        colors: HashMap<Rgb<u8>, usize>,
        alphas: HashMap<u8, usize>,
    },
    HistogramLuma16 {
        colors: HashMap<Luma<u16>, usize>,
    },
    HistogramLumaA16 {
        colors: HashMap<Luma<u16>, usize>,
        alphas: HashMap<u16, usize>,
    },
    HistogramRgb16 {
        colors: HashMap<Rgb<u16>, usize>,
    },
    HistogramRgba16 {
        colors: HashMap<Rgb<u16>, usize>,
        alphas: HashMap<u16, usize>,
    },
}

impl DynamicHistogram {
    /// Bit depth required to represent the input image's color info.
    /// Not necessarily the input image's bit depth.
    pub fn color_bit_depth(&self) -> usize {
        match self {
            DynamicHistogram::HistogramLuma8 { colors } => colors.len().next_power_of_two(),
            DynamicHistogram::HistogramLumaA8 { colors, .. } => colors.len().next_power_of_two(),
            DynamicHistogram::HistogramRgb8 { colors } => colors.len().next_power_of_two(),
            DynamicHistogram::HistogramRgba8 { colors, .. } => colors.len().next_power_of_two(),
            DynamicHistogram::HistogramLuma16 { colors } => colors.len().next_power_of_two(),
            DynamicHistogram::HistogramLumaA16 { colors, .. } => colors.len().next_power_of_two(),
            DynamicHistogram::HistogramRgb16 { colors } => colors.len().next_power_of_two(),
            DynamicHistogram::HistogramRgba16 { colors, .. } => colors.len().next_power_of_two(),
        }
    }

    /// Bit depth required to represent the input image's alpha channel.
    /// Not necessarily the input image's alpha bit depth.
    /// Returns `0` if the input image has no alpha channel.
    pub fn alpha_bit_depth(&self) -> usize {
        match self {
            DynamicHistogram::HistogramLuma8 { .. } => 0,
            DynamicHistogram::HistogramLumaA8 { colors: _, alphas } => {
                alphas.len().next_power_of_two()
            }
            DynamicHistogram::HistogramRgb8 { .. } => 0,
            DynamicHistogram::HistogramRgba8 { colors: _, alphas } => {
                alphas.len().next_power_of_two()
            }
            DynamicHistogram::HistogramLuma16 { .. } => 0,
            DynamicHistogram::HistogramLumaA16 { colors: _, alphas } => {
                alphas.len().next_power_of_two()
            }
            DynamicHistogram::HistogramRgb16 { .. } => 0,
            DynamicHistogram::HistogramRgba16 { colors: _, alphas } => {
                alphas.len().next_power_of_two()
            }
        }
    }
}

#[derive(Debug)]
pub enum DynamicHistogramError {
    /// Known float or totally unknown image format.
    /// (Floats cannot be trivially histogrammed since they don't implement Hash and Eq.)
    UnsupportedImageFormat,
}

impl TryFrom<DynamicImage> for DynamicHistogram {
    type Error = DynamicHistogramError;

    fn try_from(value: DynamicImage) -> Result<Self, Self::Error> {
        let h = match value {
            DynamicImage::ImageLuma8(image_buffer) => Self::HistogramLuma8 {
                colors: count_colors(image_buffer),
            },
            DynamicImage::ImageLumaA8(image_buffer) => {
                let (colors, alphas) = count_colors_alphas(image_buffer);
                Self::HistogramLumaA8 { colors, alphas }
            }
            DynamicImage::ImageRgb8(image_buffer) => Self::HistogramRgb8 {
                colors: count_colors(image_buffer),
            },
            DynamicImage::ImageRgba8(image_buffer) => {
                let (colors, alphas) = count_colors_alphas(image_buffer);
                Self::HistogramRgba8 { colors, alphas }
            }
            DynamicImage::ImageLuma16(image_buffer) => Self::HistogramLuma16 {
                colors: count_colors(image_buffer),
            },
            DynamicImage::ImageLumaA16(image_buffer) => {
                let (colors, alphas) = count_colors_alphas(image_buffer);
                Self::HistogramLumaA16 { colors, alphas }
            }
            DynamicImage::ImageRgb16(image_buffer) => Self::HistogramRgb16 {
                colors: count_colors(image_buffer),
            },
            DynamicImage::ImageRgba16(image_buffer) => {
                let (colors, alphas) = count_colors_alphas(image_buffer);
                Self::HistogramRgba16 { colors, alphas }
            }
            _ => return Err(Self::Error::UnsupportedImageFormat),
        };
        Ok(h)
    }
}

fn count_colors<P, V>(image_buffer: ImageBuffer<P, V>) -> HashMap<P, usize>
where
    P: Pixel + Hash + Eq,
    V: Deref<Target = [P::Subpixel]>,
{
    let mut colors = HashMap::<P, usize>::new();
    for color in image_buffer.pixels() {
        if let Some(count) = colors.get_mut(color) {
            *count += 1;
        } else {
            colors.insert(*color, 1);
        }
    }
    return colors;
}

fn count_colors_alphas<P, C, V>(
    image_buffer: ImageBuffer<P, V>,
) -> (HashMap<C, usize>, HashMap<P::Subpixel, usize>)
where
    P: Pixel + Split<C>,
    P::Subpixel: Hash + Eq,
    C: Pixel + Hash + Eq,
    V: Deref<Target = [P::Subpixel]>,
{
    let mut colors = HashMap::<C, usize>::new();
    let mut alphas = HashMap::<P::Subpixel, usize>::new();
    for pixel in image_buffer.pixels() {
        let (color, alpha) = pixel.split();
        if let Some(count) = colors.get_mut(&color) {
            *count += 1;
        } else {
            colors.insert(color, 1);
        }
        if let Some(count) = alphas.get_mut(&alpha) {
            *count += 1;
        } else {
            alphas.insert(alpha, 1);
        }
    }
    return (colors, alphas);
}

/// A pixel type that can be split into color (or luma) and alpha components.
trait Split<C>: Pixel
where
    C: Pixel,
{
    fn split(self) -> (C, Self::Subpixel);
}

// Need to use concrete subpixel types here because `image::traits::Enlargeable` is not a public trait.

impl Split<Luma<u8>> for LumaA<u8> {
    fn split(self) -> (Luma<u8>, Self::Subpixel) {
        return (Luma(self.0[..1].try_into().unwrap()), self.0[1]);
    }
}

impl Split<Luma<u16>> for LumaA<u16> {
    fn split(self) -> (Luma<u16>, Self::Subpixel) {
        return (Luma(self.0[..1].try_into().unwrap()), self.0[1]);
    }
}

impl Split<Rgb<u8>> for Rgba<u8> {
    fn split(self) -> (Rgb<u8>, Self::Subpixel) {
        return (Rgb(self.0[..3].try_into().unwrap()), self.0[3]);
    }
}

impl Split<Rgb<u16>> for Rgba<u16> {
    fn split(self) -> (Rgb<u16>, Self::Subpixel) {
        return (Rgb(self.0[..3].try_into().unwrap()), self.0[3]);
    }
}
