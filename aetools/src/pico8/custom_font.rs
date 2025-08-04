use crate::pico8::custom_character::CustomCharacter;
use anyhow::{anyhow, bail, Result};
use image::{GenericImageView, ImageReader};
use std::path::Path;

pub const TINY_FONT_FIRST_CHAR: u8 = b' ';
pub const TINY_FONT_LAST_CHAR: u8 = b'~';
pub const TINY_FONT_INTERLINE_SPACING: u8 = 1;
pub const TINY_FONT_KERNING: u8 = 1;
pub const TINY_FONT_CHAR_WIDTHS: &[u32] = &[
    3, // space
    1, // !
    3, // "
    5, // #
    3, // $
    5, // %
    4, // &
    1, // '
    2, // (
    2, // )
    5, // *
    3, // +
    2, // ,
    2, // -
    1, // .
    3, // /
    3, // 0
    2, // 1
    3, // 2
    3, // 3
    3, // 4
    3, // 5
    3, // 6
    3, // 7
    3, // 8
    3, // 9
    1, // :
    2, // ;
    3, // <
    2, // =
    3, // >
    3, // ?
    3, // @
    3, // A
    3, // B
    3, // C
    3, // D
    3, // E
    3, // F
    3, // G
    3, // H
    3, // I
    3, // J
    3, // K
    3, // L
    3, // M
    3, // N
    3, // O
    3, // P
    3, // Q
    3, // R
    3, // S
    3, // T
    3, // U
    3, // V
    3, // W
    3, // X
    3, // Y
    3, // Z
    2, // [
    3, // \
    2, // ]
    3, // ^
    3, // _
    2, // `
    3, // a
    3, // b
    3, // c
    3, // d
    3, // e
    2, // f
    3, // g
    3, // h
    1, // i
    2, // j
    3, // k
    2, // l
    3, // m
    3, // n
    3, // o
    3, // p
    3, // q
    2, // r
    3, // s
    3, // t
    3, // u
    3, // v
    3, // w
    3, // x
    3, // y
    3, // z
    3, // {
    1, // |
    3, // }
    5, // ~
];

/// With adjustments, can represent characters of widths 1 thru 8 inclusive.
const DEFAULT_WIDTH: u8 = 5;

/// PICO-8 uses the first 16 characters for control.
const CUSTOM_FONT_CHARS: usize = 240;

/// https://www.lexaloffle.com/dl/docs/pico-8_manual.html#Custom_Font
pub struct CustomFont {
    pub name: String,
    width: u8,
    /// Alternate width for characters 128 and up.
    width_hi_chars: u8,
    height: u8,
    offset_x: u8,
    offset_y: u8,
    apply_size_adjustments: bool,
    apply_tabs_relative_to_cursor_home: bool,
    tab_width: u8,
    size_adjustments: [SizeAdjustment; CUSTOM_FONT_CHARS],
    chars: [CustomCharacter; CUSTOM_FONT_CHARS],
}

impl CustomFont {
    pub fn load(
        png_path: &Path,
        threshold: u8,
        first_char: u8,
        last_char: u8,
        interline_spacing: u8,
        kerning: u8,
        char_widths: &[u32],
    ) -> Result<Self> {
        let name = png_path
            .file_stem()
            .ok_or(anyhow!("Couldn't get file stem for PNG path"))?
            .to_string_lossy()
            .to_string();
        let image = ImageReader::open(png_path)?.decode()?;
        if image.height() > 8 {
            bail!("font is too tall: {}", image.height());
        }
        let num_chars = last_char as usize - first_char as usize + 1;
        if num_chars > CUSTOM_FONT_CHARS {
            bail!("font has too many characters: {num_chars}");
        }
        if char_widths.len() > num_chars {
            bail!(
                "font has {num_chars} characters but {} character widths were provided",
                char_widths.len()
            );
        }
        for width in char_widths {
            if *width > 8 {
                bail!("font has character that is too wide: {width} pixels");
            }
        }

        let mut chars = std::array::from_fn(|_| CustomCharacter::default());
        let mut size_adjustments = std::array::from_fn(|_| SizeAdjustment::default());
        let mut x = 0u32;
        for (i, width) in char_widths.iter().enumerate() {
            let char_index = (first_char - 0x10) as usize + i;
            chars[char_index] =
                CustomCharacter::from_image(*image.view(x, 0, *width, image.height()), threshold)?;
            size_adjustments[char_index] = SizeAdjustment::from(
                (*width as i8) + (kerning as i8) - (DEFAULT_WIDTH as i8),
                false,
            )
            .ok_or(anyhow!("character {i} width out of adjustment range"))?;
            x += width;
        }

        Ok(Self {
            name,
            width: DEFAULT_WIDTH,
            width_hi_chars: DEFAULT_WIDTH,
            height: image.height() as u8 + interline_spacing,
            offset_x: 0,
            offset_y: 0,
            apply_size_adjustments: true,
            apply_tabs_relative_to_cursor_home: false,
            tab_width: DEFAULT_WIDTH,
            size_adjustments,
            chars,
        })
    }

    /// Generates a bunch of fixed-width rectangles as a test font.
    pub fn rectangles() -> Self {
        Self {
            name: "rectangle_font".to_string(),
            width: 8,
            width_hi_chars: 8,
            height: 8,
            offset_x: 0,
            offset_y: 0,
            apply_size_adjustments: false,
            apply_tabs_relative_to_cursor_home: false,
            tab_width: 8,
            size_adjustments: std::array::from_fn(|_| SizeAdjustment::default()),
            chars: std::array::from_fn(|_| CustomCharacter::rectangle()),
        }
    }

    pub fn lua_src(&self) -> String {
        let mut bytes = [0u8; 2048];
        bytes[0] = self.width;
        bytes[1] = self.width_hi_chars;
        bytes[2] = self.height;
        bytes[3] = self.offset_x;
        bytes[4] = self.offset_y;
        if self.apply_size_adjustments {
            bytes[5] |= 0x1;
        }
        if self.apply_tabs_relative_to_cursor_home {
            bytes[5] |= 0x2;
        }
        bytes[6] = self.tab_width;
        // bytes[7] is unused

        for size_adjustment_pair_index in 0..(self.size_adjustments.len() / 2) {
            let offset = 8 + size_adjustment_pair_index;
            let size_adjustment_index = 2 * size_adjustment_pair_index;
            let pair = self.size_adjustments[size_adjustment_index].nibble()
                | self.size_adjustments[size_adjustment_index + 1].nibble() << 4;
            bytes[offset] = pair;
        }

        for (char_index, char) in self.chars.iter().enumerate() {
            let offset = 0x80 + 8 * char_index;
            for (i, b) in char.bytes.iter().enumerate() {
                bytes[offset + i] = *b;
            }
        }

        let mut src = format!("{}_data = {{\n", self.name);
        for chunk in bytes.chunks_exact(4) {
            let &[a, b, c, d] = chunk else {
                panic!("somebody goofed");
            };
            let line = format!(" 0x{d:02x}{c:02x}.{b:02x}{a:02x},\n");
            src.push_str(&line);
        }
        src.push_str("}\n");
        src
    }
}

struct SizeAdjustment {
    width: i8,
    y_adjust: bool,
}

impl SizeAdjustment {
    fn from(width: i8, y_adjust: bool) -> Option<Self> {
        if width < -4 || width > 3 {
            return None;
        }
        Some(Self { width, y_adjust })
    }

    fn nibble(&self) -> u8 {
        let mut b = self.width.cast_unsigned() & 0x7;
        if self.y_adjust {
            b |= 0b1000;
        }
        b
    }
}

impl Default for SizeAdjustment {
    fn default() -> Self {
        Self {
            width: 0,
            y_adjust: false,
        }
    }
}
