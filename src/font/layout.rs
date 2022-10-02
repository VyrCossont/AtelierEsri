use crate::font::Font;
use crate::wasm4;

pub struct TypewriterText<'a> {
    font: &'a Font<'a>,
    colors: u16,
    text: String,
    /// Map character slice ends to byte slice ends for [`text`].
    byte_slice_ends: Vec<usize>,
}

impl TypewriterText<'_> {
    pub fn new<'a>(font: &'a Font, colors: u16, text: &str, width: u32) -> TypewriterText<'a> {
        let mut output_text = String::new();
        let mut is_first_line = true;
        for line in text.lines() {
            let mut words = line.split_whitespace();
            let mut output_line = if let Some(word) = words.next() {
                // A new line always takes at least one word to guarantee progress.
                String::from(word)
            } else {
                // If it has no words, it's a blank line.
                if is_first_line {
                    is_first_line = false;
                } else {
                    output_text.push('\n');
                }
                continue;
            };
            for word in words {
                let mut extended_line = output_line.clone();
                extended_line.push(' ');
                extended_line.push_str(word);
                if font.metrics(&extended_line).0 <= width {
                    output_line = extended_line;
                } else {
                    if is_first_line {
                        is_first_line = false;
                    } else {
                        output_text.push('\n');
                    }
                    output_text.push_str(&output_line);
                    output_line = String::from(word);
                }
            }
            if is_first_line {
                is_first_line = false;
            } else {
                output_text.push('\n');
            }
            output_text.push_str(&output_line);
        }
        output_text.shrink_to_fit();

        let mut char_slice_ends = Vec::<usize>::with_capacity(text.len() + 1);
        for (byte_pos, _) in output_text.char_indices() {
            char_slice_ends.push(byte_pos);
        }
        char_slice_ends.push(output_text.len());
        char_slice_ends.shrink_to_fit();

        TypewriterText {
            font,
            colors,
            text: output_text,
            byte_slice_ends: char_slice_ends,
        }
    }

    pub fn char_count(&self) -> usize {
        self.text.chars().count()
    }

    pub fn draw(&self, num_chars: usize, x: i32, y: i32) {
        let byte_slice_end = self.byte_slice_ends[num_chars];
        unsafe { *wasm4::DRAW_COLORS = self.colors }
        self.font.text(&self.text[..byte_slice_end], x, y);
    }
}
