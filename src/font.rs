use crate::asset_data;
use crate::wasm4;
use std::cmp::max;
use std::str::from_utf8_unchecked;

const TINY_FONT_SPRITE_WIDTH: u32 = 7;
const TINY_FONT_SPRITE_HEIGHT: u32 = 7;
const TINY_FONT_SPRITES_PER_ROW: u32 = asset_data::TINY_FONT_WIDTH / TINY_FONT_SPRITE_WIDTH;
const TINY_FONT_DEFAULT_WIDTH: i32 = 3;
const TINY_FONT_DEFAULT_HEIGHT: i32 = 5;
const TINY_FONT_LINE_HEIGHT: i32 = TINY_FONT_DEFAULT_HEIGHT;
const TINY_FONT_SPACE_WIDTH: i32 = TINY_FONT_DEFAULT_WIDTH;
const TINY_FONT_DEFAULT_START_X_OFFSET: u32 = 2;
const TINY_FONT_DEFAULT_START_Y_OFFSET: u32 = 1;
const TINY_FONT_KERNING: i32 = 1;
const TINY_FONT_LINE_SPACING: i32 = 1;

pub enum Font<'a> {
    BuiltIn,
    Proportional {
        sprite_data: &'a [u8],
        sprite_width: u32,
        sprite_height: u32,
        sprites_per_row: u32,
        default_width: i32,
        default_height: i32,
        line_height: i32,
        space_width: i32,
        default_start_x_offset: u32,
        default_start_y_offset: u32,
        kerning: i32,
        line_spacing: i32,
    },
}

impl Font<'_> {
    /// Draw text.
    pub fn text(&self, s: &str, x: i32, y: i32) {
        match self {
            Self::BuiltIn => Self::builtin_text(s, x, y),
            _ => todo!(),
        }
    }

    /// Return the bounding box that a drawn string would have.
    pub fn metrics(&self, s: &str) -> (u32, u32) {
        match self {
            Self::BuiltIn => Self::builtin_metrics(s),
            _ => todo!(),
        }
    }

    fn builtin_text(s: &str, x: i32, y: i32) {
        let bytes = Self::builtin_text_to_bytes(s);
        let extd_ascii_text = unsafe { from_utf8_unchecked(&bytes) };
        wasm4::text(extd_ascii_text, x, y);
    }

    fn builtin_metrics(s: &str) -> (u32, u32) {
        let bytes = Self::builtin_text_to_bytes(s);
        if bytes.is_empty() {
            return (0, 0);
        }
        let mut current_col = 0;
        let mut max_col = 0;
        let mut lines = 1;
        for c in bytes {
            if c == '\n' as u8 {
                lines += 1;
                max_col = max(max_col, current_col);
                current_col = 0;
            } else {
                current_col += 1;
            }
        }
        max_col = max(max_col, current_col);
        return (8 * max_col, 8 * lines);
    }

    /// WASM-4's `text()` actually takes a 1-byte character set based on ISO-8859-1,
    /// with a few extra symbols, so we have to translate Unicode characters to that.
    fn builtin_text_to_bytes(s: &str) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(s.len());
        for c in s.chars() {
            match c {
                // ASCII:
                // TODO: is \r handled differently?
                '\n' => bytes.push(c as u8),
                ' '..='~' => bytes.push(c as u8),

                // WASM-4 puts two low dots where U+007F DELETE would be.
                // Might be an under-dieresis, but U+0324 COMBINING DIAERESIS BELOW
                // has no non-combining equivalent like U+00A8 ¨ DIAERESIS,
                // so we'll use U+2025 TWO DOT LEADER to represent this glyph in Unicode text.
                '‥' => bytes.push(0x7f),

                // WASM-4 button characters:
                // Ignore text and emoji variation selectors to make it easier to type buttons.
                '\u{fe0e}' | '\u{fe0f}' => (),

                // Note that buttons 1 and 2 are documented as buttons A and B in
                // https://wasm4.org/docs/guides/text#special-characters
                // but rendered with X and Z respectively, so we'll accept either of these:
                // U+1F167 NEGATIVE CIRCLED LATIN CAPITAL LETTER X
                // U+1F170 NEGATIVE SQUARED LATIN CAPITAL LETTER A (aka blood type emoji A)
                '🅧' | '🅰' => bytes.push(0x80),

                // U+1F169 NEGATIVE CIRCLED LATIN CAPITAL LETTER Z
                // U+1F171 NEGATIVE SQUARED LATIN CAPITAL LETTER B (aka blood type emoji B)
                '🅩' | '🅱' => bytes.push(0x81),

                // U+2B05 LEFTWARDS BLACK ARROW
                '⬅' => bytes.push(0x84),

                // U+27A1 BLACK RIGHTWARDS ARROW
                // (used by Apple emoji picker but renders funny without emoji variation selector because it's from Dingbats)
                // U+2B95 RIGHTWARDS BLACK ARROW
                '➡' | '⮕' => bytes.push(0x85),

                // U+2B06 UPWARDS BLACK ARROW
                '⬆' => bytes.push(0x86),

                // U+2B07 DOWNWARDS BLACK ARROW
                '⬇' => bytes.push(0x87),

                // Map all Latin-1 Supplement characters to their ISO-8859-1 equivalents:
                '\u{a0}'..='ÿ' => bytes.push(c as u8),

                _ => {
                    let mut panic_msg =
                        String::from("Unicode character not supported by built-in font: ");
                    panic_msg.extend(c.escape_default());
                    wasm4::trace(panic_msg);
                    panic!();
                }
            }
        }
        bytes
    }
}

/// Draw text with custom font from `my-font-dark.png`.
pub fn ftext(s: &str, x: i32, y: i32) {
    fcore(s, x, y, true);
}

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
            cx += TINY_FONT_KERNING;
        }
        kern_next = true;
        match c {
            ' ' => {
                cx += TINY_FONT_SPACE_WIDTH as i32 + TINY_FONT_KERNING;
            }
            '\n' => {
                cx_max = max(cx, cx_max);
                cx = x;
                cy += TINY_FONT_LINE_HEIGHT + TINY_FONT_LINE_SPACING;
                kern_next = false;
            }
            _ => {
                if let Ok(g) = Glyph::try_from(c) {
                    let (src_x, src_y) = g.src();
                    let w = g.width();
                    if draw {
                        wasm4::blit_sub(
                            &asset_data::TINY_FONT,
                            cx,
                            cy,
                            w as u32,
                            TINY_FONT_DEFAULT_HEIGHT as u32,
                            src_x,
                            src_y,
                            asset_data::TINY_FONT_WIDTH,
                            asset_data::TINY_FONT_FLAGS,
                        );
                    }
                    cx += w;
                } else {
                    // Missing character replacement block.
                    if draw {
                        wasm4::rect(
                            cx,
                            cy,
                            TINY_FONT_DEFAULT_WIDTH as u32,
                            TINY_FONT_DEFAULT_HEIGHT as u32,
                        );
                    }
                    cx += TINY_FONT_DEFAULT_WIDTH;
                }
            }
        }
    }
    cx_max = max(cx, cx_max);
    (cx_max, cy + TINY_FONT_LINE_HEIGHT)
}

enum FontError {
    UnsupportedCharacter(char),
}

struct Glyph(u32);

impl Glyph {
    /// Returns texture coordinates for use with `blit_sub`.
    fn src(&self) -> (u32, u32) {
        let r = self.0 % TINY_FONT_SPRITES_PER_ROW;
        let c = self.0 / TINY_FONT_SPRITES_PER_ROW;
        let x = r * TINY_FONT_SPRITE_WIDTH
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
            _ => TINY_FONT_DEFAULT_START_X_OFFSET,
        };
        let y = c * TINY_FONT_SPRITE_HEIGHT + TINY_FONT_DEFAULT_START_Y_OFFSET;
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
            _ => TINY_FONT_DEFAULT_WIDTH,
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
