use crate::font;
use crate::font::{Font, TypewriterText};
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
        // font::BUILTIN,
        font::TINY,
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
