use crate::mac::palette::{DEFAULT_4_BIT_COLOR_PALETTE, DEFAULT_8_BIT_COLOR_PALETTE};
use crate::mac::resource::icon::io::{apply_quickdraw_palette, IconIO, SizedIcon};
use crate::mac::resource::TypedResource;
use crate::mac::OSType;
use binrw::binrw;
use image::DynamicImage;

/// Indexed color image with fixed palette structure shared by other `ic??` icons.
trait PalettedIcon<const NUM_COLORS: usize>: SizedIcon {
    const ICON_PALETTE: &'static [[u16; 3]; NUM_COLORS];
    fn image_data(&self) -> &[u8];

    fn image(&self) -> Option<DynamicImage> {
        Some(DynamicImage::ImageRgb16(apply_quickdraw_palette(
            Self::ICON_SIZE,
            self.image_data(),
            Self::ICON_PALETTE,
        )))
    }
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

impl SizedIcon for Icon4BitLarge {
    const ICON_SIZE: (u32, u32) = Self::ICON_SIZE_LARGE;
}

impl PalettedIcon<16> for Icon4BitLarge {
    const ICON_PALETTE: &'static [[u16; 3]; 16] = &DEFAULT_4_BIT_COLOR_PALETTE;

    fn image_data(&self) -> &[u8] {
        &self.image_data
    }
}

impl IconIO for Icon4BitLarge {
    fn image(&self) -> Option<DynamicImage> {
        <Self as PalettedIcon<16>>::image(self)
    }

    fn mask(&self) -> Option<DynamicImage> {
        None
    }
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

impl SizedIcon for Icon4BitSmall {
    const ICON_SIZE: (u32, u32) = Self::ICON_SIZE_SMALL;
}

impl PalettedIcon<16> for Icon4BitSmall {
    const ICON_PALETTE: &'static [[u16; 3]; 16] = &DEFAULT_4_BIT_COLOR_PALETTE;

    fn image_data(&self) -> &[u8] {
        &self.image_data
    }
}

impl IconIO for Icon4BitSmall {
    fn image(&self) -> Option<DynamicImage> {
        <Self as PalettedIcon<16>>::image(self)
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
pub struct Icon4BitMini {
    pub image_data: [u8; 96],
}

impl TypedResource for Icon4BitMini {
    const OS_TYPE: OSType = *b"icm4";
}

impl SizedIcon for Icon4BitMini {
    const ICON_SIZE: (u32, u32) = Self::ICON_SIZE_MINI;
}

impl PalettedIcon<16> for Icon4BitMini {
    const ICON_PALETTE: &'static [[u16; 3]; 16] = &DEFAULT_4_BIT_COLOR_PALETTE;

    fn image_data(&self) -> &[u8] {
        &self.image_data
    }
}

impl IconIO for Icon4BitMini {
    fn image(&self) -> Option<DynamicImage> {
        <Self as PalettedIcon<16>>::image(self)
    }

    fn mask(&self) -> Option<DynamicImage> {
        None
    }
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

impl SizedIcon for Icon8BitLarge {
    const ICON_SIZE: (u32, u32) = Self::ICON_SIZE_LARGE;
}

impl PalettedIcon<256> for Icon8BitLarge {
    const ICON_PALETTE: &'static [[u16; 3]; 256] = &DEFAULT_8_BIT_COLOR_PALETTE;

    fn image_data(&self) -> &[u8] {
        &self.image_data
    }
}

impl IconIO for Icon8BitLarge {
    fn image(&self) -> Option<DynamicImage> {
        <Self as PalettedIcon<256>>::image(self)
    }

    fn mask(&self) -> Option<DynamicImage> {
        None
    }
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

impl SizedIcon for Icon8BitSmall {
    const ICON_SIZE: (u32, u32) = Self::ICON_SIZE_SMALL;
}

impl PalettedIcon<256> for Icon8BitSmall {
    const ICON_PALETTE: &'static [[u16; 3]; 256] = &DEFAULT_8_BIT_COLOR_PALETTE;

    fn image_data(&self) -> &[u8] {
        &self.image_data
    }
}

impl IconIO for Icon8BitSmall {
    fn image(&self) -> Option<DynamicImage> {
        <Self as PalettedIcon<256>>::image(self)
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

impl SizedIcon for Icon8BitMini {
    const ICON_SIZE: (u32, u32) = Self::ICON_SIZE_MINI;
}

impl PalettedIcon<256> for Icon8BitMini {
    const ICON_PALETTE: &'static [[u16; 3]; 256] = &DEFAULT_8_BIT_COLOR_PALETTE;

    fn image_data(&self) -> &[u8] {
        &self.image_data
    }
}

impl IconIO for Icon8BitMini {
    fn image(&self) -> Option<DynamicImage> {
        <Self as PalettedIcon<256>>::image(self)
    }

    fn mask(&self) -> Option<DynamicImage> {
        None
    }
}
