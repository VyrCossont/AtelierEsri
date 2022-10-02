use crate::gfx::{Unisprite, UnispriteData};

const UNISPRITE: &Unisprite = &Unisprite {
    w: 16,
    h: 16,
    data: UnispriteData::L0 { color: 2 },
};

pub fn init() {}

/// Returns whether we should keep running the intro.
pub fn update() -> bool {
    UNISPRITE.draw(3, 3);

    UNISPRITE.draw(157, 157);

    true
}
