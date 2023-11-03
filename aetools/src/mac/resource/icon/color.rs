use crate::mac::resource::TypedResource;
use crate::mac::OSType;
use binrw::binrw;

// TODO: (Vyr) implement `cicn` support
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
