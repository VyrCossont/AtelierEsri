use crate::asset_data;
use crate::font::{fmetrics, ftext, Font};
use crate::gfx::{thick_hline, thick_line, Lo5SplitSprite};
use crate::gfx_data;
use crate::map_data;
use crate::wasm4;

const ANIMATION_CYCLE_LEN: u32 = 2 * 5 * 60;
static mut ANIMATION_CLOCK: u32 = 0;

struct Character<'a> {
    name: &'a str,
    bio: [&'a str; 4],
    sprite: &'a Lo5SplitSprite<'a>,
    palette: [u32; 4],
    map_x: i32,
    map_y: i32,
}

const CHARACTERS: [Character; 3] = [
    Character {
        name: "Esri",
        bio: [
            "alchemist",
            "impulsive party girl",
            "petty criminal, kleptomaniac,",
            "aspiring drug dealer",
        ],
        sprite: &gfx_data::ESRI,
        // https://lospec.com/palette-list/ice-cream-gb
        palette: [0xfffff6d3, 0xfff9a875, 0xffeb6b6f, 0xff7c3f58],
        map_x: 0,
        map_y: 20,
    },
    Character {
        name: "Allie",
        bio: [
            "blonde in body and soul",
            "entirely too cheerful",
            "would never do a crime on purpose",
            "constantly doing crimes by accident",
        ],
        sprite: &gfx_data::ALLIE,
        // https://lospec.com/palette-list/muddysand
        palette: [0xffe6d69c, 0xffb4a56a, 0xff7b7162, 0xff393829],
        map_x: 200,
        map_y: 110,
    },
    Character {
        name: "Sae",
        bio: [
            "failed alchemist",
            "perpetually half asleep",
            "party conscience",
            "natural top",
        ],
        sprite: &gfx_data::SAE,
        // https://lospec.com/palette-list/2bit-demichrome
        palette: [0xffe9efec, 0xffa0a08b, 0xff555568, 0xff211e20],
        map_x: 120,
        map_y: 50,
    },
];

/// Returns whether we should keep running the intro.
pub fn update() -> bool {
    unsafe {
        if *wasm4::GAMEPAD1 & wasm4::BUTTON_1 != 0 {
            return false;
        }
    }

    let animation_clock = unsafe { ANIMATION_CLOCK };

    let character = &CHARACTERS[(animation_clock / 200) as usize];
    for (i, c) in character.palette.into_iter().enumerate() {
        unsafe { (&mut *wasm4::PALETTE)[i] = c }
    }

    // Cheat past our map clipping problem by drawing the bricks on top of the map.
    // TODO: map clipping
    let bg_split_y: u32 = 80;
    let map_cycle = (animation_clock as i32 % 200) / 10;
    for layer in [&map_data::VILLAGE_FLOOR, &map_data::VILLAGE_BUILDINGS] {
        map_data::VILLAGE_FLOOR.draw(
            0,
            bg_split_y as i32,
            character.map_x + map_cycle,
            character.map_y + map_cycle,
            wasm4::SCREEN_SIZE,
            wasm4::SCREEN_SIZE - bg_split_y,
        );
    }

    // Loops 15 times in the animation cycle assuming 8x? tile..
    let bg_cycle = (animation_clock / 5) % asset_data::BG_BRICKS_WIDTH;
    // Intentionally drawing one more column than would fill the screen.
    unsafe { *wasm4::DRAW_COLORS = 0x21 }
    for x in (0..=wasm4::SCREEN_SIZE).step_by(asset_data::BG_BRICKS_WIDTH as usize) {
        for y in (0..bg_split_y).step_by(asset_data::BG_BRICKS_HEIGHT as usize) {
            wasm4::blit(
                &asset_data::BG_BRICKS,
                (x - bg_cycle) as i32,
                y as i32,
                asset_data::BG_BRICKS_WIDTH,
                asset_data::BG_BRICKS_HEIGHT,
                asset_data::BG_BRICKS_FLAGS,
            );
        }
    }

    let character_bg_start_x = wasm4::SCREEN_SIZE - character.sprite.w + 6;
    let character_bg_start_y = wasm4::SCREEN_SIZE - character.sprite.h - 2;
    // draw character overlay fill
    unsafe { *wasm4::DRAW_COLORS = 1 }
    for y in character_bg_start_y..wasm4::SCREEN_SIZE {
        let x = character_bg_start_x - (y - character_bg_start_y) / 4;
        wasm4::hline(x as i32, y as i32, wasm4::SCREEN_SIZE - x);
    }
    // draw character overlay border
    {
        unsafe { *wasm4::DRAW_COLORS = 3 }
        let thickness = 2;
        let x1 = character_bg_start_x as i32;
        let y1 = character_bg_start_y as i32;
        let x2 = x1 - (wasm4::SCREEN_SIZE as i32 - y1) / 4;
        let y2 = wasm4::SCREEN_SIZE as i32;
        thick_hline(x1, y1, wasm4::SCREEN_SIZE - x1 as u32, thickness);
        thick_line(x1, y1, x2, y2, thickness, thickness);
    }

    character.sprite.blit(
        (wasm4::SCREEN_SIZE - character.sprite.w) as i32,
        (wasm4::SCREEN_SIZE - character.sprite.h) as i32,
        0,
    );

    shadow_text(character.name, 5, 5);

    for (i, t) in character.bio.into_iter().enumerate() {
        let x = 5;
        let y = 20 + (i as i32) * 15;
        shadow_text("➡️", x, y - 1);
        shadow_ftext(t, x + 10, y);
    }

    unsafe {
        ANIMATION_CLOCK = (animation_clock + 1) % ANIMATION_CYCLE_LEN;
    }

    true
}

fn shadow_text(t: &str, x: i32, y: i32) {
    let (mw, mh) = Font::BuiltIn.metrics(t);
    unsafe { *wasm4::DRAW_COLORS = 0x11 }
    wasm4::rect(x - 1, y - 1, mw + 2, mh + 2);
    unsafe { *wasm4::DRAW_COLORS = 0x3 }
    Font::BuiltIn.text(t, x + 1, y + 1);
    unsafe { *wasm4::DRAW_COLORS = 0x4 }
    Font::BuiltIn.text(t, x, y);
}

fn shadow_ftext(t: &str, x: i32, y: i32) {
    let (mw, mh) = fmetrics(t);
    unsafe { *wasm4::DRAW_COLORS = 0x11 }
    wasm4::rect(x - 1, y - 1, mw + 2, mh + 2);
    unsafe { *wasm4::DRAW_COLORS = 0x430 }
    ftext(t, x, y);
}
