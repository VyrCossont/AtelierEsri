use crate::asset_data::item_unisprite::AXE;
use crate::gfx::Sprite;
use crate::wasm4;

pub fn init() {}

/// Returns whether we should keep running the intro.
pub fn update() -> bool {
    AXE.draw(80, 80);

    // Test transparency
    unsafe { *wasm4::DRAW_COLORS = 0x0022 };
    wasm4::rect(0, 0, 16, 16);
    AXE.draw(0, 0);

    // Test clipping
    AXE.draw(150, 150);

    true
}
