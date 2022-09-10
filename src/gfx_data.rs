use crate::asset_data;
use crate::gfx::{CharacterSprite, Lo5SplitSprite};
use crate::wasm4;

pub const ESRI: Lo5SplitSprite = Lo5SplitSprite {
    w: asset_data::ESRI_LO4_WIDTH,
    h: asset_data::ESRI_LO4_HEIGHT,
    lo4: &asset_data::ESRI_LO4,
    hi2: &asset_data::ESRI_HI2,
};

pub const ALLIE: Lo5SplitSprite = Lo5SplitSprite {
    w: asset_data::ALLIE_LO4_WIDTH,
    h: asset_data::ALLIE_LO4_HEIGHT,
    lo4: &asset_data::ALLIE_LO4,
    hi2: &asset_data::ALLIE_HI2,
};

pub const SAE: Lo5SplitSprite = Lo5SplitSprite {
    w: asset_data::SAE_LO4_WIDTH,
    h: asset_data::SAE_LO4_HEIGHT,
    lo4: &asset_data::SAE_LO4,
    hi2: &asset_data::SAE_HI2,
};

pub const KMRPG: Lo5SplitSprite = Lo5SplitSprite {
    w: asset_data::KENNEY_MONOCHROMERPG_EXTENDED_LO4_WIDTH,
    h: asset_data::KENNEY_MONOCHROMERPG_EXTENDED_LO4_HEIGHT,
    lo4: &asset_data::KENNEY_MONOCHROMERPG_EXTENDED_LO4,
    hi2: &asset_data::KENNEY_MONOCHROMERPG_EXTENDED_HI2,
};

pub const GUNGIRL: CharacterSprite = CharacterSprite {
    image_w: asset_data::GUNGIRL_WIDTH,
    image_h: asset_data::GUNGIRL_HEIGHT,
    image: &asset_data::GUNGIRL,
    draw_colors: 0x4320,
    sprite_w: 16,
    walk_cycle_length: 4,
    orientation_starts_flags: [
        (0, asset_data::GUNGIRL_FLAGS),
        (4, asset_data::GUNGIRL_FLAGS),
        (16, asset_data::GUNGIRL_FLAGS),
        (4, asset_data::GUNGIRL_FLAGS | wasm4::BLIT_FLIP_X),
        (0, asset_data::GUNGIRL_FLAGS | wasm4::BLIT_FLIP_X),
        (8, asset_data::GUNGIRL_FLAGS | wasm4::BLIT_FLIP_X),
        (12, asset_data::GUNGIRL_FLAGS),
        (8, asset_data::GUNGIRL_FLAGS),
    ],
};
