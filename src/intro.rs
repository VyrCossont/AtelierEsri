use crate::gfx::Sprite;
use aesprite::Unisprite;

const UNISPRITE: &Unisprite<&[u8]> = &Unisprite {
    w: 16,
    h: 16,
    luma: &[],
    alpha: &[],
};

pub fn init() {}

/// Returns whether we should keep running the intro.
pub fn update() -> bool {
    UNISPRITE.draw(3, 3);

    UNISPRITE.draw(157, 157);

    true
}
