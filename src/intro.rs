use crate::font::Font;
use crate::font_data;
use crate::wasm4;

const DEMO_TEXT: &str = "That's Harris all over - so ready to take the burden of everything himself, and put it on the backs of other people.

He always reminds me of my poor Uncle Podger. You never saw such a commotion up and down a house, in all your life, as when my Uncle Podger undertook to do a job. A picture would have come home from the frame-maker's, and be standing in the dining-room, waiting to be put up; and Aunt Podger would ask what was to be done with it, and Uncle Podger would say:

\"Oh, you leave that to me. Don't you, any of you, worry yourselves about that. I'll do all that.\"

And then he would take off his coat, and begin. He would send the girl out for sixpen'orth of nails, and then one of the boys after her to tell her what size to get; and, from that, he would gradually work down, and start the whole house.";

struct State<'a> {
    tt_text: TypewriterText<'a>,
    animation_cycle_len: usize,
    animation_clock: usize,
}

static mut STATE: Option<State> = None;

const MARGIN: i32 = 4;
const FRAMES_PER_CHARACTER: usize = 3;

pub fn init() {
    let tt_text = TypewriterText::new(
        // font_data::BUILTIN,
        font_data::TINY,
        // 0x0003,
        0x230,
        DEMO_TEXT,
        wasm4::SCREEN_SIZE - 2 * MARGIN as u32,
    );
    let animation_cycle_len = FRAMES_PER_CHARACTER * tt_text.char_count();
    let state = State {
        tt_text,
        animation_cycle_len,
        animation_clock: 0,
    };
    unsafe {
        STATE = Some(state);
    }
}

/// Returns whether we should keep running the intro.
pub fn update() -> bool {
    unsafe {
        if *wasm4::GAMEPAD1 & wasm4::BUTTON_1 != 0 {
            return false;
        }
    }

    let state = unsafe { STATE.as_mut() }.expect("intro::init() probably not called");

    state
        .tt_text
        .draw(state.animation_clock / FRAMES_PER_CHARACTER, MARGIN, MARGIN);

    state.animation_clock = (state.animation_clock + 1) % state.animation_cycle_len;

    true
}

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
