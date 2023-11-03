pub mod icon;

use crate::mac::resource::icon::*;
use crate::mac::OSType;
use binrw::binrw;

/// Every known resource plus a catch-all.
#[binrw]
#[brw(big)]
#[br(import(os_type: OSType, data_len: usize))]
pub enum Resource {
    #[br(pre_assert(os_type == Icon1BitLargeOldest::OS_TYPE))]
    ICON(Icon1BitLargeOldest),
    #[br(pre_assert(os_type == Icon1BitSmallMaskedOldest::OS_TYPE))]
    SICN(Icon1BitSmallMaskedOldest),
    #[br(pre_assert(os_type == IconColorMaskedOldest::OS_TYPE))]
    CICN(IconColorMaskedOldest),

    #[br(pre_assert(os_type == Icon1BitLargeMasked::OS_TYPE))]
    ICNHash(Icon1BitLargeMasked),
    #[br(pre_assert(os_type == Icon1BitSmallMasked::OS_TYPE))]
    ICSHash(Icon1BitSmallMasked),
    #[br(pre_assert(os_type == Icon1BitMiniMasked::OS_TYPE))]
    ICMHash(Icon1BitMiniMasked),

    #[br(pre_assert(os_type == Icon4BitLarge::OS_TYPE))]
    ICL4(Icon4BitLarge),
    #[br(pre_assert(os_type == Icon4BitSmall::OS_TYPE))]
    ICS4(Icon4BitSmall),
    #[br(pre_assert(os_type == Icon4BitMini::OS_TYPE))]
    ICM4(Icon4BitMini),

    #[br(pre_assert(os_type == Icon8BitLarge::OS_TYPE))]
    ICL8(Icon8BitLarge),
    #[br(pre_assert(os_type == Icon8BitSmall::OS_TYPE))]
    ICS8(Icon8BitSmall),
    #[br(pre_assert(os_type == Icon8BitMini::OS_TYPE))]
    ICM8(Icon8BitMini),

    Unknown {
        os_type: OSType,
        #[br(count = data_len)]
        data: Vec<u8>,
    },
}

/// Associate an `OSType` with its resource structure.
pub trait TypedResource {
    const OS_TYPE: OSType;
}
