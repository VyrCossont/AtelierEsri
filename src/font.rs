include!(concat!(env!("OUT_DIR"), "/assets.rs"));
use crate::wasm4::*;
use std::cmp::max;

const MY_FONT_SPRITE_WIDTH: u32 = 7;
const MY_FONT_SPRITE_HEIGHT: u32 = 7;
const MY_FONT_SPRITES_PER_ROW: u32 = MY_FONT_DARK_WIDTH / MY_FONT_SPRITE_WIDTH;
const MY_FONT_DEFAULT_WIDTH: i32 = 3;
const MY_FONT_DEFAULT_HEIGHT: i32 = 5;
const MY_FONT_LINE_HEIGHT: i32 = MY_FONT_DEFAULT_HEIGHT;
const MY_FONT_SPACE_WIDTH: i32 = MY_FONT_DEFAULT_WIDTH;
const MY_FONT_DEFAULT_START_X_OFFSET: u32 = 2;
const MY_FONT_DEFAULT_START_Y_OFFSET: u32 = 1;
const MY_FONT_KERNING: i32 = 1;
const MY_FONT_LINE_SPACING: i32 = 1;

/// Draw text with custom font from `my-font-dark.png`.
pub fn ftext(s: &str, x: i32, y: i32) {
    fcore(s, x, y, true);
}

/// Return the bounding box that a string would use.
pub fn fmetrics(s: &str) -> (u32, u32) {
    let (x, y) = fcore(s, 0, 0, false);
    (x as u32, y as u32)
}

/// Common functionality of `ftext` and `fmetrics`.
fn fcore(s: &str, x: i32, y: i32, draw: bool) -> (i32, i32) {
    let mut cx = x;
    let mut cy = y;
    // Keep track of end of widest line.
    let mut cx_max = x;
    // Keep track of when inter-letter spacing needs to be added,
    // so we don't count spurious end-of-line kerning in metrics boxes.
    let mut kern_next = false;
    for c in s.chars() {
        if kern_next {
            cx += MY_FONT_KERNING;
        }
        kern_next = true;
        match c {
            ' ' => {
                cx += MY_FONT_SPACE_WIDTH as i32 + MY_FONT_KERNING;
            }
            '\n' => {
                cx_max = max(cx, cx_max);
                cx = x;
                cy += MY_FONT_LINE_HEIGHT + MY_FONT_LINE_SPACING;
                kern_next = false;
            }
            _ => {
                if let Ok(g) = Glyph::try_from(c) {
                    let (src_x, src_y) = g.src();
                    let w = g.width();
                    if draw {
                        blit_sub(
                            &MY_FONT_DARK,
                            cx,
                            cy,
                            w as u32,
                            MY_FONT_DEFAULT_HEIGHT as u32,
                            src_x,
                            src_y,
                            MY_FONT_DARK_WIDTH,
                            MY_FONT_DARK_FLAGS,
                        );
                    }
                    cx += w;
                } else {
                    // Missing character replacement block.
                    if draw {
                        rect(
                            cx,
                            cy,
                            MY_FONT_DEFAULT_WIDTH as u32,
                            MY_FONT_DEFAULT_HEIGHT as u32,
                        );
                    }
                    cx += MY_FONT_DEFAULT_WIDTH;
                }
            }
        }
    }
    cx_max = max(cx, cx_max);
    (cx_max, cy + MY_FONT_LINE_HEIGHT)
}

enum FontError {
    UnsupportedCharacter(char),
}

struct Glyph(u32);

impl Glyph {
    /// Returns texture coordinates for use with `blit_sub`.
    fn src(&self) -> (u32, u32) {
        let r = self.0 % MY_FONT_SPRITES_PER_ROW;
        let c = self.0 / MY_FONT_SPRITES_PER_ROW;
        let x = r * MY_FONT_SPRITE_WIDTH
            + match self.0 {
            34 /* 'i' */ => 3,
            37 /* 'l' */ => 3,
            53 /* '1' */ => 3,
            64 /* ';' */ => 3,
            65 /* ':' */ => 3,
            67 /* '!' */ => 3,
            70 /* '\'' */ => 3,
            71 /* '*' */ => 1,
            78 /* ')' */ => 3,
            _ => MY_FONT_DEFAULT_START_X_OFFSET,
        };
        let y = c * MY_FONT_SPRITE_HEIGHT + MY_FONT_DEFAULT_START_Y_OFFSET;
        (x, y)
    }

    fn width(&self) -> i32 {
        match self.0 {
            34 /* 'i' */ => 1,
            35 /* 'j' */ => 2,
            37 /* 'l' */ => 2,
            43 /* 'r' */ => 2,
            53 /* '1' */ => 2,
            62 /* '.' */ => 1,
            63 /* ',' */ => 2,
            64 /* ';' */ => 2,
            65 /* ':' */ => 1,
            67 /* '!' */ => 1,
            68 /* '-' */ => 2,
            70 /* '\'' */ => 1,
            71 /* '*' */ => 5,
            77 /* '(' */ => 2,
            78 /* ')' */ => 2,
            _ => MY_FONT_DEFAULT_WIDTH,
        }
    }
}

impl TryFrom<char> for Glyph {
    type Error = FontError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let u = c as u32;
        let i: u32 = match c {
            'A'..='Z' => u - 'A' as u32,
            'a'..='z' => u - 'a' as u32 + 26,
            '0'..='9' => u - '0' as u32 + 52,
            '.' => 62,
            ',' => 63,
            ';' => 64,
            ':' => 65,
            '?' => 66,
            '!' => 67,
            '-' => 68,
            '_' => 69,
            '\'' => 70,
            '*' => 71,
            '"' => 72,
            '\\' => 73,
            '/' => 74,
            '<' => 75,
            '>' => 76,
            '(' => 77,
            ')' => 78,
            '@' => 79,
            _ => return Err(FontError::UnsupportedCharacter(c)),
        };
        Ok(Glyph(i))
    }
}

/// WASM-4 `text()` actually expects ASCII with some nonstandard escapes, not UTF-8.
pub fn btext(t: &[u8], x: i32, y: i32) {
    let extd_ascii_text = unsafe { std::str::from_utf8_unchecked(t) };
    text(extd_ascii_text, x, y);
}
