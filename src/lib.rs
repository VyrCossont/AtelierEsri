include!(concat!(env!("OUT_DIR"), "/assets.rs"));

#[cfg(feature = "buddy-alloc")]
mod alloc;
mod audio;
mod audio_data;
mod font;
mod gfx;
mod gfx_data;
mod wasm4;

use crate::audio::{music, music_update};
use crate::font::{btext, fmetrics, ftext};
use crate::gfx::SplitSprite;
use crate::gfx_data::{ALLIE, ESRI, SAE};
use wasm4::*;

const ANIMATION_CYCLE_LEN: u32 = 2 * 5 * 60;
static mut ANIMATION_CLOCK: u32 = 0;

struct Character<'a> {
    name: &'a str,
    bio: [&'a str; 4],
    sprite: &'a SplitSprite<'a>,
    palette: [u32; 4],
    mid_bg: bool,
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
        sprite: &ESRI,
        // https://lospec.com/palette-list/ice-cream-gb
        palette: [0xfffff6d3, 0xfff9a875, 0xffeb6b6f, 0xff7c3f58],
        mid_bg: false,
    },
    Character {
        name: "Allie",
        bio: [
            "blonde in body and soul",
            "entirely too cheerful",
            "would never do a crime on purpose",
            "constantly doing crimes by accident",
        ],
        sprite: &ALLIE,
        // https://lospec.com/palette-list/muddysand
        palette: [0xffe6d69c, 0xffb4a56a, 0xff7b7162, 0xff393829],
        mid_bg: false,
    },
    Character {
        name: "Sae",
        bio: [
            "failed alchemist",
            "perpetually half asleep",
            "party conscience",
            "natural top",
        ],
        sprite: &SAE,
        // https://lospec.com/palette-list/2bit-demichrome
        palette: [0xffe9efec, 0xffa0a08b, 0xff555568, 0xff211e20],
        mid_bg: false,
    },
];

#[no_mangle]
fn start() {
    audio::init();
    music(0);
}

#[no_mangle]
fn update() {
    music_update();

    let animation_clock = unsafe { ANIMATION_CLOCK };

    let character = &CHARACTERS[(animation_clock / 200) as usize];
    for (i, c) in character.palette.into_iter().enumerate() {
        unsafe { (&mut *PALETTE)[i] = c }
    }

    if character.mid_bg {
        unsafe { *DRAW_COLORS = 0x0023 }
    } else {
        unsafe { *DRAW_COLORS = 0x0012 }
    }
    // Loops 15 times in the animation cycle assuming 8x? tile..
    let bg_cycle = (animation_clock / 5) % BG_BRICKS_WIDTH;
    // Intentionally drawing one more column than would fill the screen.
    for x in (0..=SCREEN_SIZE).step_by(BG_BRICKS_WIDTH as usize) {
        for y in (0..SCREEN_SIZE).step_by(BG_BRICKS_HEIGHT as usize) {
            blit(
                &BG_BRICKS,
                (x - bg_cycle) as i32,
                y as i32,
                BG_BRICKS_WIDTH,
                BG_BRICKS_HEIGHT,
                BG_BRICKS_FLAGS,
            );
        }
    }

    character.sprite.blit(
        (SCREEN_SIZE - character.sprite.w) as i32,
        (SCREEN_SIZE - character.sprite.h) as i32,
        0,
    );

    shadow_text(character.name, 10, 10);

    for (i, t) in character.bio.into_iter().enumerate() {
        let x = 10;
        let y = 25 + (i as i32) * 15;
        shadow_btext(b"\x85", x, y);
        shadow_ftext(t, x + 10, y);
    }

    unsafe {
        ANIMATION_CLOCK = (animation_clock + 1) % ANIMATION_CYCLE_LEN;
    }
}

fn shadow_text(t: &str, x: i32, y: i32) {
    unsafe { *DRAW_COLORS = 0x3 }
    text(t, x + 1, y + 1);
    unsafe { *DRAW_COLORS = 0x4 }
    text(t, x, y);
}

fn shadow_btext(t: &[u8], x: i32, y: i32) {
    unsafe { *DRAW_COLORS = 0x3 }
    btext(t, x + 1, y + 1);
    unsafe { *DRAW_COLORS = 0x4 }
    btext(t, x, y);
}

fn shadow_ftext(t: &str, x: i32, y: i32) {
    let (mw, mh) = fmetrics(t);
    unsafe { *DRAW_COLORS = 0x11 }
    rect(x - 1, y - 1, mw + 2, mh + 2);
    unsafe { *DRAW_COLORS = 0x430 }
    ftext(t, x, y);
}
