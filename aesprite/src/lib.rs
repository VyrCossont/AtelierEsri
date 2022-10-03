use std::ops::Index;

pub struct Unisprite<'a> {
    pub w: i32,
    pub h: i32,
    pub data: UnispriteData<'a>,
}

pub type WASM4PaletteIndex = u8;

pub enum UnispriteData<U: Index<usize, Output = u8>> {
    /// Just a color.
    /// This is just a rectangle, really, but it's good for testing the blitter.
    L0 { fill: WASM4PaletteIndex },
    /// Color and 1-bit mask.
    L0A1 { fill: WASM4PaletteIndex, alpha: U },
    /// 1-bit indexed with mini-palette mapping it to full WASM-4 2-bit palette.
    L1 {
        minipalette: UnispriteMinipalette,
        indexes: U,
    },
    /// 1-bit indexed with mini-palette and mask.
    L1A1 {
        minipalette: UnispriteMinipalette,
        indexes: U,
        alpha: U,
    },
    /// 2-bit indexed using full WASM-4 2-bit palette.
    L2 { luma: U },
    /// 2-bit indexed with mask.
    L2A1 { luma: U, alpha: U },
}

type Q<'a> = UnispriteData<&'a [u8]>;
type X = UnispriteData<Vec<u8>>;

pub enum UnispriteMinipalette {
    /// Map indexes 0 and 1 to WASM-4 palette indexes 0 and 1.
    P01,
    /// Map indexes 0 and 1 to WASM-4 palette indexes 0 and 2.
    P02,
    /// Map indexes 0 and 1 to WASM-4 palette indexes 0 and 3.
    P03,
    /// Map indexes 0 and 1 to WASM-4 palette indexes 1 and 2.
    P12,
    /// Map indexes 0 and 1 to WASM-4 palette indexes 1 and 3.
    P13,
    /// Map indexes 0 and 1 to WASM-4 palette indexes 2 and 3.
    P23,
}
