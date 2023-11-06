use crate::histogram::DynamicHistogram;
use crate::mac::resource::icon::io::{expand_grays, IconIO, IconIOError, SizedIcon};
use crate::mac::resource::TypedResource;
use crate::mac::OSType;
use binrw::binrw;
use image::DynamicImage;

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

impl SizedIcon for Icon1BitLargeOldest {
    const ICON_SIZE: (u32, u32) = Self::ICON_SIZE_LARGE;
}

impl IconIO for Icon1BitLargeOldest {
    fn image(&self) -> Option<DynamicImage> {
        Some(DynamicImage::ImageLuma8(expand_grays::<1>(
            Self::ICON_SIZE,
            &self.image_data,
            true,
        )))
    }

    fn mask(&self) -> Option<DynamicImage> {
        None
    }

    fn try_from(image: DynamicImage) -> Result<Self, IconIOError> {
        let histogram = DynamicHistogram::try_from(image).map_err(|e| IconIOError::Histogram(e))?;

        if histogram.color_bit_depth() > 1 {
            return Err(IconIOError::Depth);
        }
        if histogram.alpha_bit_depth() > 1 {
            return Err(IconIOError::Depth);
        }

        match histogram {
            DynamicHistogram::HistogramLuma8 { colors } => {}
            DynamicHistogram::HistogramLumaA8 { colors, alphas } => {}
            DynamicHistogram::HistogramRgb8 { colors } => {}
            DynamicHistogram::HistogramRgba8 { colors, alphas } => {}
            DynamicHistogram::HistogramLuma16 { colors } => {}
            DynamicHistogram::HistogramLumaA16 { colors, alphas } => {}
            DynamicHistogram::HistogramRgb16 { colors } => {}
            DynamicHistogram::HistogramRgba16 { colors, alphas } => {}
        }

        Ok(Self {
            // TODO
            image_data: [0u8; 128],
        })
    }
}
