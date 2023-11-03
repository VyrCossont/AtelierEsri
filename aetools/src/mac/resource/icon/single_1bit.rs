use crate::mac::resource::icon::io::{expand_grays, IconIO, SizedIcon};
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
}
