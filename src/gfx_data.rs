include!(concat!(env!("OUT_DIR"), "/assets.rs"));

use crate::gfx::SplitSprite;

pub const ESRI: SplitSprite = SplitSprite {
    w: ESRI_COLORS_01_WIDTH,
    h: ESRI_COLORS_01_HEIGHT,
    layers: [&ESRI_COLORS_01, &ESRI_COLORS_23],
};

pub const ALLIE: SplitSprite = SplitSprite {
    w: ALLIE_COLORS_01_WIDTH,
    h: ALLIE_COLORS_01_HEIGHT,
    layers: [&ALLIE_COLORS_01, &ALLIE_COLORS_23],
};

pub const SAE: SplitSprite = SplitSprite {
    w: SAE_COLORS_01_WIDTH,
    h: SAE_COLORS_01_HEIGHT,
    layers: [&SAE_COLORS_01, &SAE_COLORS_23],
};
