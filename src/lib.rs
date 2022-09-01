include!(concat!(env!("OUT_DIR"), "/assets.rs"));

#[cfg(feature = "buddy-alloc")]
mod alloc;
mod font;
mod wasm4;
use font::*;
use wasm4::*;

struct Wasm4Tone {
    frequency: u32,
    duration: u32,
    volume: u32,
    flags: u32,
}

#[derive(Default)]
struct SfxPlayback<'a> {
    frames_per_tone: u8,
    frame_counter: u8,
    tone_counter: u8,
    tones: &'a [Wasm4Tone],
}

impl SfxPlayback<'_> {
    fn playing(&self) -> bool {
        (self.tone_counter as usize) < self.tones.len()
    }

    fn update(&mut self) {
        if !self.playing() {
            return;
        }
        if self.frame_counter == 0 {
            let sfx_tone = &self.tones[self.tone_counter as usize];
            if sfx_tone.volume > 0 {
                tone(
                    sfx_tone.frequency,
                    sfx_tone.duration,
                    sfx_tone.volume,
                    sfx_tone.flags,
                );
                trace("tone");
            }
        }
        self.frame_counter += 1;
        if self.frame_counter == self.frames_per_tone {
            self.frame_counter = 0;
            self.tone_counter += 1;
        }
    }
}

const C_TEST: [Wasm4Tone; 7] = [
    Wasm4Tone {
        frequency: 130,
        duration: 8,
        volume: 71,
        flags: 2,
    },
    Wasm4Tone {
        frequency: 261,
        duration: 8,
        volume: 71,
        flags: 2,
    },
    Wasm4Tone {
        frequency: 523,
        duration: 8,
        volume: 71,
        flags: 2,
    },
    Wasm4Tone {
        frequency: 1046,
        duration: 8,
        volume: 71,
        flags: 2,
    },
    Wasm4Tone {
        frequency: 2093,
        duration: 8,
        volume: 71,
        flags: 2,
    },
    Wasm4Tone {
        frequency: 65,
        duration: 8,
        volume: 0,
        flags: 2,
    },
    Wasm4Tone {
        frequency: 440,
        duration: 8,
        volume: 71,
        flags: 2,
    },
];

const SFX0: *mut SfxPlayback = 0x19a0 as *mut SfxPlayback;

#[no_mangle]
fn start() {
    unsafe {
        *SFX0 = SfxPlayback {
            frames_per_tone: 8,
            frame_counter: 0,
            tone_counter: 0,
            tones: &C_TEST,
        }
    }
}

#[no_mangle]
fn update() {
    let msg = "Hello\nfrom\nRust!";

    let (w, h) = fmetrics(msg);

    unsafe { *DRAW_COLORS = 0x0320 }
    ftext(msg, 80 - (w as i32 / 2), 80 - (h as i32 / 2));

    unsafe {
        let sfx = &mut *SFX0;
        sfx.update();
    }
}
