use crate::font::{fmetrics, ftext, Font};
use crate::gfx::{thick_hline, thick_line, SplitSprite};
use crate::gfx_data;
use crate::map_data;
use crate::wasm4::{
    blit, hline, rect, BUTTON_DOWN, BUTTON_LEFT, BUTTON_RIGHT, BUTTON_UP, DRAW_COLORS, GAMEPAD1,
    PALETTE, SCREEN_SIZE,
};
use std::cmp::{max, min};

static mut MAP_X: i32 = 0;
static mut MAP_Y: i32 = 0;

pub fn update() {
    let (mut map_x, mut map_y) = unsafe { (MAP_X, MAP_Y) };
    unsafe { *DRAW_COLORS = 0x1234 }
    map_data::VILLAGE.draw(0, 0, map_x, map_y, SCREEN_SIZE, SCREEN_SIZE);

    let gamepad = unsafe { *GAMEPAD1 };
    if gamepad & BUTTON_LEFT != 0 {
        map_x -= 1;
    }
    if gamepad & BUTTON_RIGHT != 0 {
        map_x += 1;
    }
    if gamepad & BUTTON_UP != 0 {
        map_y -= 1;
    }
    if gamepad & BUTTON_DOWN != 0 {
        map_y += 1;
    }
    map_x = max(
        0,
        min(
            (map_data::VILLAGE.dimensions().0 - SCREEN_SIZE) as i32,
            map_x,
        ),
    );
    map_y = max(
        0,
        min(
            (map_data::VILLAGE.dimensions().1 - SCREEN_SIZE) as i32,
            map_y,
        ),
    );

    unsafe { (MAP_X, MAP_Y) = (map_x, map_y) }
}
