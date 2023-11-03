use crate::mac::resource::icon::io::{expand_grays, IconIO, SizedIcon};
use crate::mac::resource::TypedResource;
use crate::mac::OSType;
use binrw::binrw;
use image::DynamicImage;

/// 1-bit icon list structure shared by `SICN` and `ICN#`.
trait Icon1BitList: SizedIcon {
    fn icon_list(&self) -> Vec<&[u8]>;

    fn image(&self) -> Option<DynamicImage> {
        self.icon_list()
            .get(0)
            .map(|data| DynamicImage::ImageLuma8(expand_grays::<1>(Self::ICON_SIZE, data, true)))
    }

    fn mask(&self) -> Option<DynamicImage> {
        self.icon_list()
            .get(1)
            .map(|data| DynamicImage::ImageLuma8(expand_grays::<1>(Self::ICON_SIZE, data, true)))
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

impl SizedIcon for Icon1BitSmallMaskedOldest {
    const ICON_SIZE: (u32, u32) = Self::ICON_SIZE_SMALL;
}

impl Icon1BitList for Icon1BitSmallMaskedOldest {
    fn icon_list(&self) -> Vec<&[u8]> {
        self.image_datas.iter().map(|x| x.as_slice()).collect()
    }
}

impl IconIO for Icon1BitSmallMaskedOldest {
    fn image(&self) -> Option<DynamicImage> {
        <Self as Icon1BitList>::image(self)
    }

    fn mask(&self) -> Option<DynamicImage> {
        <Self as Icon1BitList>::mask(self)
    }
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

impl SizedIcon for Icon1BitLargeMasked {
    const ICON_SIZE: (u32, u32) = Self::ICON_SIZE_LARGE;
}

impl Icon1BitList for Icon1BitLargeMasked {
    fn icon_list(&self) -> Vec<&[u8]> {
        self.image_datas.iter().map(|x| x.as_slice()).collect()
    }
}

impl IconIO for Icon1BitLargeMasked {
    fn image(&self) -> Option<DynamicImage> {
        <Self as Icon1BitList>::image(self)
    }

    fn mask(&self) -> Option<DynamicImage> {
        <Self as Icon1BitList>::mask(self)
    }
}
