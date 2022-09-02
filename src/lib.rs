include!(concat!(env!("OUT_DIR"), "/assets.rs"));

#[cfg(feature = "buddy-alloc")]
mod alloc;
mod font;
mod wasm4;
use font::*;
use wasm4::*;

struct Tone {
    frequency: u32,
    duration: u32,
    volume: u32,
    flags: u32,
}

struct Sfx<'a> {
    frames_per_tone: u8,
    loop_restart: Option<usize>,
    tones: &'a [Tone],
}

struct SfxPlayback<'a> {
    frame_counter: u8,
    tone_index: usize,
    sfx: &'a Sfx<'a>,
}

impl SfxPlayback<'_> {
    fn playing(&self) -> bool {
        self.tone_index < self.sfx.tones.len()
    }

    /// Returns whether it will continue to play.
    /// Always returns true for looped SFX.
    fn update(&mut self) -> bool {
        if !self.playing() {
            if let Some(loop_restart) = self.sfx.loop_restart {
                self.frame_counter = 0;
                self.tone_index = loop_restart;
            } else {
                return false;
            }
        }

        if self.frame_counter == 0 {
            let sfx_tone = &self.sfx.tones[self.tone_index];
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
        if self.frame_counter == self.sfx.frames_per_tone {
            self.frame_counter = 0;
            self.tone_index += 1;
        }

        return true;
    }
}

const SFX_DATA: &[Sfx] = &[Sfx {
    frames_per_tone: 8,
    loop_restart: Some(0),
    tones: &[
        Tone {
            frequency: 130,
            duration: 8,
            volume: 71,
            flags: 2,
        },
        Tone {
            frequency: 261,
            duration: 8,
            volume: 71,
            flags: 2,
        },
        Tone {
            frequency: 523,
            duration: 8,
            volume: 71,
            flags: 2,
        },
        Tone {
            frequency: 1046,
            duration: 8,
            volume: 71,
            flags: 2,
        },
        Tone {
            frequency: 2093,
            duration: 8,
            volume: 71,
            flags: 2,
        },
        Tone {
            frequency: 65,
            duration: 8,
            volume: 0,
            flags: 2,
        },
        Tone {
            frequency: 440,
            duration: 8,
            volume: 71,
            flags: 2,
        },
        Tone {
            frequency: 65,
            duration: 8,
            volume: 0,
            flags: 2,
        },
        Tone {
            frequency: 65,
            duration: 8,
            volume: 0,
            flags: 2,
        },
        Tone {
            frequency: 65,
            duration: 8,
            volume: 0,
            flags: 2,
        },
        Tone {
            frequency: 65,
            duration: 8,
            volume: 0,
            flags: 2,
        },
        Tone {
            frequency: 65,
            duration: 8,
            volume: 0,
            flags: 2,
        },
        Tone {
            frequency: 65,
            duration: 8,
            volume: 0,
            flags: 2,
        },
        Tone {
            frequency: 65,
            duration: 8,
            volume: 0,
            flags: 2,
        },
        Tone {
            frequency: 65,
            duration: 8,
            volume: 0,
            flags: 2,
        },
        Tone {
            frequency: 65,
            duration: 8,
            volume: 0,
            flags: 2,
        },
    ],
}];

const SFX0: *mut SfxPlayback = 0x19a0 as *mut SfxPlayback;

#[no_mangle]
fn start() {
    unsafe {
        *SFX0 = SfxPlayback {
            frame_counter: 0,
            tone_index: 0,
            sfx: &SFX_DATA[0],
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
