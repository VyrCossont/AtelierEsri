use crate::asset_data;
use crate::gfx::{CharacterSprite, SplitSprite};
use crate::wasm4;

pub const ESRI: SplitSprite = SplitSprite {
    w: asset_data::ESRI_COLORS_01_WIDTH,
    h: asset_data::ESRI_COLORS_01_HEIGHT,
    layers: [&asset_data::ESRI_COLORS_01, &asset_data::ESRI_COLORS_23],
};

pub const ALLIE: SplitSprite = SplitSprite {
    w: asset_data::ALLIE_COLORS_01_WIDTH,
    h: asset_data::ALLIE_COLORS_01_HEIGHT,
    layers: [&asset_data::ALLIE_COLORS_01, &asset_data::ALLIE_COLORS_23],
};

pub const SAE: SplitSprite = SplitSprite {
    w: asset_data::SAE_COLORS_01_WIDTH,
    h: asset_data::SAE_COLORS_01_HEIGHT,
    layers: [&asset_data::SAE_COLORS_01, &asset_data::SAE_COLORS_23],
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
