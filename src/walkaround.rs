use crate::gfx::Orientation;
use crate::gfx_data;
use crate::map_data;
use crate::wasm4::{
    BUTTON_DOWN, BUTTON_LEFT, BUTTON_RIGHT, BUTTON_UP, DRAW_COLORS, GAMEPAD1, SCREEN_SIZE,
};
use std::cmp::{max, min};

static mut PLAYER_X: i32 = 0;
static mut PLAYER_Y: i32 = 0;
// Can't do `::default()` in a const context. Tragic.
static mut PLAYER_O: Orientation = Orientation::S;
static mut PLAYER_W: usize = 0;

pub fn update() {
    let (mut player_x, mut player_y, mut player_o, mut player_w) =
        unsafe { (PLAYER_X, PLAYER_Y, PLAYER_O, PLAYER_W) };

    let (map_w, map_h) = map_data::VILLAGE_FLOOR.dimensions();
    let map_x = max(0, min((map_w - SCREEN_SIZE) as i32, player_x));
    let map_y = max(0, min((map_h - SCREEN_SIZE) as i32, player_y));

    unsafe { *DRAW_COLORS = 0x1234 }
    map_data::VILLAGE_FLOOR.draw(0, 0, map_x, map_y, SCREEN_SIZE, SCREEN_SIZE);

    let gamepad = unsafe { *GAMEPAD1 };
    let mut heading_x = 0;
    let mut heading_y = 0;
    if gamepad & BUTTON_LEFT != 0 {
        heading_x -= 1;
    }
    if gamepad & BUTTON_RIGHT != 0 {
        heading_x += 1;
    }
    if gamepad & BUTTON_UP != 0 {
        heading_y -= 1;
    }
    if gamepad & BUTTON_DOWN != 0 {
        heading_y += 1;
    }
    if heading_x != 0 || heading_y != 0 {
        player_o = Orientation::from((heading_x, heading_y));
        player_w += 1;
        player_w %= gfx_data::GUNGIRL.walk_cycle_length;
    } else {
        player_w = 0;
    }
    player_x = max(
        0,
        min(
            (map_w - gfx_data::GUNGIRL.sprite_w) as i32,
            player_x + heading_x,
        ),
    );
    player_y = max(
        0,
        min(
            (map_h - gfx_data::GUNGIRL.image_h) as i32,
            player_y + heading_y,
        ),
    );

    let player_screen_x = player_x - map_x;
    let player_screen_y = player_y - map_y;

    gfx_data::GUNGIRL.draw(player_screen_x, player_screen_y, player_w, player_o);

    // TODO: this won't actually work, we need to draw actors between map rows
    map_data::VILLAGE_BUILDINGS.draw(0, 0, map_x, map_y, SCREEN_SIZE, SCREEN_SIZE);

    unsafe { (PLAYER_X, PLAYER_Y, PLAYER_O, PLAYER_W) = (player_x, player_y, player_o, player_w) }
}
