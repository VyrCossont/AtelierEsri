include!(concat!(env!("OUT_DIR"), "/assets.rs"));

use crate::wasm4::{blit, BLIT_2BPP, DRAW_COLORS};

/// Sprite composed of 2 2BPP sprites,
/// the 1st with transparent color, color 0, color 1, unused color,
/// the 2nd with transparent color, color 2, color 3, unused color,
/// letting us draw a 4-color sprite with transparency.
/// TODO: use 1 2BPP sprite for transparent, color 0, color 1, color 2
///     and 1 1BPP sprite for transparent, color 3?
pub struct SplitSprite<'a> {
    pub w: u32,
    pub h: u32,
    pub layers: [&'a [u8]; 2],
}

impl SplitSprite<'_> {
    pub fn blit(&self, x: i32, y: i32, flags: u32) {
        unsafe { *DRAW_COLORS = 0x0043 }
        blit(self.layers[0], x, y, self.w, self.h, flags | BLIT_2BPP);
        unsafe { *DRAW_COLORS = 0x0021 }
        blit(self.layers[1], x, y, self.w, self.h, flags | BLIT_2BPP);
    }
}
