use crate::asset_data;
use crate::gfx::{CharacterSprite, Cursor, Lo5SplitSprite};
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

pub const BAR_COPPER: &Lo5SplitSprite = &Lo5SplitSprite {
    w: asset_data::ITEM_BAR_COPPER_LO4_WIDTH,
    h: asset_data::ITEM_BAR_COPPER_LO4_HEIGHT,
    lo4: &asset_data::ITEM_BAR_COPPER_LO4,
    hi2: &asset_data::ITEM_BAR_COPPER_HI2,
};

pub const ORE_COPPER: &Lo5SplitSprite = &Lo5SplitSprite {
    w: asset_data::ITEM_ORE_COPPER_LO4_WIDTH,
    h: asset_data::ITEM_ORE_COPPER_LO4_HEIGHT,
    lo4: &asset_data::ITEM_ORE_COPPER_LO4,
    hi2: &asset_data::ITEM_ORE_COPPER_HI2,
};

pub const WOOD: &Lo5SplitSprite = &Lo5SplitSprite {
    w: asset_data::ITEM_WOOD_LO4_WIDTH,
    h: asset_data::ITEM_WOOD_LO4_HEIGHT,
    lo4: &asset_data::ITEM_WOOD_LO4,
    hi2: &asset_data::ITEM_WOOD_HI2,
};

pub const RUBY: &Lo5SplitSprite = &Lo5SplitSprite {
    w: asset_data::ITEM_RUBY_LO4_WIDTH,
    h: asset_data::ITEM_RUBY_LO4_HEIGHT,
    lo4: &asset_data::ITEM_RUBY_LO4,
    hi2: &asset_data::ITEM_RUBY_HI2,
};

pub const SCROLL: &Lo5SplitSprite = &Lo5SplitSprite {
    w: asset_data::ITEM_SCROLL_LO4_WIDTH,
    h: asset_data::ITEM_SCROLL_LO4_HEIGHT,
    lo4: &asset_data::ITEM_SCROLL_LO4,
    hi2: &asset_data::ITEM_SCROLL_HI2,
};

pub const SAND: &Lo5SplitSprite = &Lo5SplitSprite {
    w: asset_data::ITEM_SAND_LO4_WIDTH,
    h: asset_data::ITEM_SAND_LO4_HEIGHT,
    lo4: &asset_data::ITEM_SAND_LO4,
    hi2: &asset_data::ITEM_SAND_HI2,
};

pub const BOMB: &Lo5SplitSprite = &Lo5SplitSprite {
    w: asset_data::ITEM_BOMB_LO4_WIDTH,
    h: asset_data::ITEM_BOMB_LO4_HEIGHT,
    lo4: &asset_data::ITEM_BOMB_LO4,
    hi2: &asset_data::ITEM_BOMB_HI2,
};

pub const TEST_TUBE: &Lo5SplitSprite = &Lo5SplitSprite {
    w: asset_data::ITEM_TEST_TUBE_LO4_WIDTH,
    h: asset_data::ITEM_TEST_TUBE_LO4_HEIGHT,
    lo4: &asset_data::ITEM_TEST_TUBE_LO4,
    hi2: &asset_data::ITEM_TEST_TUBE_HI2,
};

pub const WATER: &Lo5SplitSprite = &Lo5SplitSprite {
    w: asset_data::ITEM_WATER_LO4_WIDTH,
    h: asset_data::ITEM_WATER_LO4_HEIGHT,
    lo4: &asset_data::ITEM_WATER_LO4,
    hi2: &asset_data::ITEM_WATER_HI2,
};

pub const FLOWER1: &Lo5SplitSprite = &Lo5SplitSprite {
    w: asset_data::ITEM_FLOWER1_LO4_WIDTH,
    h: asset_data::ITEM_FLOWER1_LO4_HEIGHT,
    lo4: &asset_data::ITEM_FLOWER1_LO4,
    hi2: &asset_data::ITEM_FLOWER1_HI2,
};

pub const CURSOR_POINT: &Cursor = &Cursor {
    sprite: &Lo5SplitSprite {
        w: asset_data::CURSOR_POINT_LO4_WIDTH,
        h: asset_data::CURSOR_POINT_LO4_HEIGHT,
        lo4: &asset_data::CURSOR_POINT_LO4,
        hi2: &asset_data::CURSOR_POINT_HI2,
    },
    hotspot: (1, 1),
};

pub const CURSOR_TOUCH: &Cursor = &Cursor {
    sprite: &Lo5SplitSprite {
        w: asset_data::CURSOR_TOUCH_LO4_WIDTH,
        h: asset_data::CURSOR_TOUCH_LO4_HEIGHT,
        lo4: &asset_data::CURSOR_TOUCH_LO4,
        hi2: &asset_data::CURSOR_TOUCH_HI2,
    },
    hotspot: (3, 1),
};
