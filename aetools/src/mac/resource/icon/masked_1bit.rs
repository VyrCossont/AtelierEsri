use crate::mac::resource::icon::io::{expand_grays, IconIO, IconIOError, SizedIcon};
use crate::mac::resource::TypedResource;
use crate::mac::OSType;
use binrw::binrw;
use image::DynamicImage;

/// 1-bit image and mask structure shared by `ics#` and `icm#`.
trait Icon1BitMasked: SizedIcon + Sized {
    fn image_data(&self) -> &[u8];
    fn mask_data(&self) -> &[u8];

    fn image(&self) -> Option<DynamicImage> {
        Some(DynamicImage::ImageLuma8(expand_grays::<1>(
            Self::ICON_SIZE,
            self.image_data(),
            true,
        )))
    }

    fn mask(&self) -> Option<DynamicImage> {
        Some(DynamicImage::ImageLuma8(expand_grays::<1>(
            Self::ICON_SIZE,
            self.mask_data(),
            true,
        )))
    }

    fn try_from(image: DynamicImage) -> Result<Self, IconIOError> {
        todo!()
    }
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

impl SizedIcon for Icon1BitSmallMasked {
    const ICON_SIZE: (u32, u32) = Self::ICON_SIZE_SMALL;
}

impl Icon1BitMasked for Icon1BitSmallMasked {
    fn image_data(&self) -> &[u8] {
        &self.image_data
    }

    fn mask_data(&self) -> &[u8] {
        &self.image_data
    }
}

impl IconIO for Icon1BitSmallMasked {
    fn image(&self) -> Option<DynamicImage> {
        <Self as Icon1BitMasked>::image(self)
    }

    fn mask(&self) -> Option<DynamicImage> {
        <Self as Icon1BitMasked>::mask(self)
    }

    fn try_from(image: DynamicImage) -> Result<Self, IconIOError> {
        <Self as Icon1BitMasked>::try_from(image)
    }
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

impl SizedIcon for Icon1BitMiniMasked {
    const ICON_SIZE: (u32, u32) = Self::ICON_SIZE_MINI;
}

impl Icon1BitMasked for Icon1BitMiniMasked {
    fn image_data(&self) -> &[u8] {
        &self.image_data
    }

    fn mask_data(&self) -> &[u8] {
        &self.image_data
    }
}

impl IconIO for Icon1BitMiniMasked {
    fn image(&self) -> Option<DynamicImage> {
        <Self as Icon1BitMasked>::image(self)
    }

    fn mask(&self) -> Option<DynamicImage> {
        <Self as Icon1BitMasked>::mask(self)
    }

    fn try_from(image: DynamicImage) -> Result<Self, IconIOError> {
        <Self as Icon1BitMasked>::try_from(image)
    }
}
