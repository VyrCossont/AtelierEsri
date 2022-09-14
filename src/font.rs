use crate::wasm4;
use std::cmp::max;
use std::str::from_utf8_unchecked;

pub enum Font<'a> {
    BuiltIn,
    Proportional(ProportionalFont<'a>),
}

impl Font<'_> {
    /// Draw text.
    pub fn text(&self, s: &str, x: i32, y: i32) {
        match self {
            Self::BuiltIn => Self::builtin_text(s, x, y),
            Self::Proportional(font) => font.text(s, x, y),
        }
    }

    /// Return the bounding box that a drawn string would have.
    pub fn metrics(&self, s: &str) -> (u32, u32) {
        match self {
            Self::BuiltIn => Self::builtin_metrics(s),
            Self::Proportional(font) => font.metrics(s),
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

    /// [`wasm4::text`] actually takes a 1-byte character set based on ISO-8859-1,
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

/// Assumes fixed height, variable width, starting at '!', stored in horizontal strip.
/// Color 1 is assumed to be transparent,
/// color 2 the most intense and used for most of the text,
/// color 3 (if present) used for antialiasing,
/// color 4 (if present) used for even fainter antialiasing than color 3.
pub struct ProportionalFont<'a> {
    pub image_data: &'a [u8],
    pub image_width: u32,
    /// Used as line height in a lot of places.
    pub image_height: u32,
    pub image_flags: u32,
    /// Width of the space character, which is not stored in the image.
    pub space_width: i32,
    /// Horizontal space between glyphs.
    pub kerning: i32,
    /// Vertical space between lines.
    pub line_spacing: i32,
    /// Contiguous array of `src_x` values starting from '!'.
    /// Implicitly stores widths.
    /// The first entry should be 0
    /// (unless multiple fonts are packed in the same image)
    /// and the last entry is not associated with an actual character,
    /// but is just there to provide a final length.
    pub src_xs: &'a [u32],
}

impl ProportionalFont<'_> {
    pub fn text(&self, s: &str, x: i32, y: i32) {
        self.core(s, x, y, true);
    }

    pub fn metrics(&self, s: &str) -> (u32, u32) {
        self.core(s, 0, 0, false)
    }

    /// Common functionality of [`text`] and [`metrics`].
    fn core(&self, s: &str, x: i32, y: i32, draw: bool) -> (u32, u32) {
        let mut cx = x;
        let mut cy = y;
        // Keep track of end of widest line.
        let mut cx_max = x;
        // Keep track of when inter-letter spacing needs to be added,
        // so we don't count spurious end-of-line kerning in metrics boxes.
        let mut kern_next = false;
        for c in s.chars() {
            if kern_next {
                cx += self.kerning;
            }
            kern_next = true;

            match c {
                ' ' => {
                    cx += self.space_width + self.kerning;
                }

                '\n' => {
                    cx_max = max(cx, cx_max);
                    cx = x;
                    cy += self.image_height as i32 + self.line_spacing;
                    kern_next = false;
                }

                _ if c > '!' && (c as usize - '!' as usize) < self.src_xs.len() - 1 => {
                    let src_x_index = c as usize - '!' as usize;
                    let src_x = self.src_xs[src_x_index];
                    let next_src_x_index = src_x_index + 1;
                    let width = self.src_xs[next_src_x_index] - src_x;
                    if draw {
                        wasm4::blit_sub(
                            self.image_data,
                            cx,
                            cy,
                            width,
                            self.image_height,
                            src_x,
                            0,
                            self.image_width,
                            self.image_flags,
                        );
                    }
                    cx += width as i32;
                }

                _ => {
                    let mut panic_msg =
                        String::from("Unicode character not supported by proportional font: ");
                    panic_msg.extend(c.escape_default());
                    wasm4::trace(panic_msg);
                    panic!();
                }
            }
        }
        cx_max = max(cx, cx_max);
        (cx_max as u32, self.image_height + cy as u32)
    }
}
