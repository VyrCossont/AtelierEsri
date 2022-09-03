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

impl<'a> Sfx<'a> {
    fn looped(&self) -> bool {
        self.loop_restart.is_some()
    }
}

#[derive(Default)]
struct SfxPlayback<'a> {
    frame_counter: u8,
    tone_index: usize,
    sfx: Option<&'a Sfx<'a>>,
}

impl<'a> SfxPlayback<'a> {
    fn play(&mut self, sfx: &'a Sfx<'a>) {
        self.frame_counter = 0;
        self.tone_index = 0;
        self.sfx = Some(sfx);
    }

    fn stop(&mut self) {
        self.sfx = None;
    }

    fn playing(&self) -> bool {
        if let Some(sfx) = self.sfx {
            self.tone_index < sfx.tones.len()
        } else {
            false
        }
    }

    /// Returns whether it will continue to play.
    /// Always returns true for looped SFX.
    fn update(&mut self) -> bool {
        let sfx = match self.sfx {
            Some(sfx) => sfx,
            None => return false,
        };

        if !self.playing() {
            if let Some(loop_restart) = sfx.loop_restart {
                self.frame_counter = 0;
                self.tone_index = loop_restart;
            } else {
                return false;
            }
        }

        if self.frame_counter == 0 {
            let sfx_tone = &sfx.tones[self.tone_index];
            if sfx_tone.volume > 0 {
                tone(
                    sfx_tone.frequency,
                    sfx_tone.duration,
                    sfx_tone.volume,
                    sfx_tone.flags,
                );
            }
        }

        self.frame_counter += 1;
        if self.frame_counter == sfx.frames_per_tone {
            self.frame_counter = 0;
            self.tone_index += 1;
        }

        return true;
    }
}

struct Pattern<'a> {
    loop_start: bool,
    loop_back: bool,
    stop_at_end: bool,
    sfxes: &'a [&'a Sfx<'a>],
}

struct MusicPlayback<'a> {
    playing: bool,
    pattern_index: usize,
    patterns: &'a [Pattern<'a>],
    sfx_playbacks: &'a mut [&'a mut SfxPlayback<'a>],
}

impl MusicPlayback<'_> {
    /// Returns whether it will continue to play.
    /// Always returns true for looped music.
    /// See https://www.lexaloffle.com/dl/docs/pico-8_manual.html#Flow_control
    fn update(&mut self) -> bool {
        if !self.playing {
            return false;
        }

        let pattern = &self.patterns[self.pattern_index];
        let mut pattern_finished = false;
        let mut found_leftmost_nonlooping = false;
        for (sfx_playback, sfx) in self.sfx_playbacks.iter_mut().zip(pattern.sfxes) {
            let sfx_playing = sfx_playback.update();
            if !sfx.looped() && !found_leftmost_nonlooping {
                found_leftmost_nonlooping = true;
                pattern_finished = !sfx_playing;
            }
        }

        if pattern_finished {
            if self.pattern_index >= self.patterns.len() - 1 {
                self.stop();
                return false;
            } else if pattern.stop_at_end {
                self.stop();
                return false;
            } else if pattern.loop_back {
                // Find previous loop start, or go back to beginning of music data.
                self.pattern_index = self.patterns[0..=self.pattern_index]
                    .iter()
                    .enumerate()
                    .rfind(|(_, p)| p.loop_start)
                    .map(|(i, _)| i)
                    .unwrap_or(0);
            } else {
                self.pattern_index += 1;
            }
            self.start_pattern();
        }

        return true;
    }

    fn start_pattern(&mut self) {
        let sfxes = self.patterns[self.pattern_index].sfxes;
        for (sfx_playback, sfx) in self.sfx_playbacks.iter_mut().zip(sfxes) {
            sfx_playback.play(sfx);
        }
    }

    fn play(&mut self, pattern_index: usize) {
        self.stop();
        self.pattern_index = pattern_index;
        self.playing = true;
        self.start_pattern();
    }

    fn stop(&mut self) {
        self.playing = false;
        for sfx_playback in self.sfx_playbacks.iter_mut() {
            sfx_playback.stop();
        }
    }
}

//region SFX and music data

const SFX_DATA: &[Sfx] = &[
    Sfx {
        frames_per_tone: 8,
        loop_restart: None,
        tones: &[
            Tone {
                frequency: 493,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 493,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 493,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 493,
                duration: 8,
                volume: 0,
                flags: 2,
            },
            Tone {
                frequency: 493,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 493,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 493,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 493,
                duration: 8,
                volume: 0,
                flags: 2,
            },
            Tone {
                frequency: 523,
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
                frequency: 523,
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
                frequency: 587,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 587,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 587,
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
                frequency: 587,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 587,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 587,
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
                frequency: 523,
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
                frequency: 523,
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
                frequency: 493,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 493,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 493,
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
                frequency: 440,
                duration: 8,
                volume: 71,
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
        ],
    },
    Sfx {
        frames_per_tone: 8,
        loop_restart: None,
        tones: &[
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 0,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 174,
                duration: 8,
                volume: 0,
                flags: 2,
            },
        ],
    },
    Sfx {
        frames_per_tone: 8,
        loop_restart: None,
        tones: &[
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 0,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 0,
                flags: 2,
            },
        ],
    },
    Sfx {
        frames_per_tone: 8,
        loop_restart: None,
        tones: &[
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 0,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 0,
                flags: 2,
            },
        ],
    },
    Sfx {
        frames_per_tone: 8,
        loop_restart: None,
        tones: &[
            Tone {
                frequency: 391,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 391,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 391,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 493,
                duration: 8,
                volume: 0,
                flags: 2,
            },
            Tone {
                frequency: 391,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 391,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 391,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 440,
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
                frequency: 440,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 440,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 440,
                duration: 8,
                volume: 0,
                flags: 2,
            },
            Tone {
                frequency: 493,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 493,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 493,
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
                frequency: 493,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 493,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 493,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 493,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 493,
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
                frequency: 440,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 440,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 440,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 440,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 440,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 440,
                duration: 8,
                volume: 71,
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
        ],
    },
    Sfx {
        frames_per_tone: 8,
        loop_restart: None,
        tones: &[
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 0,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 174,
                duration: 8,
                volume: 0,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
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
        ],
    },
    Sfx {
        frames_per_tone: 8,
        loop_restart: None,
        tones: &[
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 0,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 0,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
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
        ],
    },
    Sfx {
        frames_per_tone: 8,
        loop_restart: None,
        tones: &[
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 0,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 0,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 0,
                flags: 2,
            },
        ],
    },
    Sfx {
        frames_per_tone: 8,
        loop_restart: None,
        tones: &[
            Tone {
                frequency: 391,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 391,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 391,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 493,
                duration: 8,
                volume: 0,
                flags: 2,
            },
            Tone {
                frequency: 391,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 391,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 391,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 440,
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
                frequency: 440,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 440,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 440,
                duration: 8,
                volume: 0,
                flags: 2,
            },
            Tone {
                frequency: 493,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 493,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 493,
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
                frequency: 440,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 440,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 440,
                duration: 8,
                volume: 71,
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
                frequency: 391,
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
                frequency: 391,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 391,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 391,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 391,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 391,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 391,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 391,
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
        ],
    },
    Sfx {
        frames_per_tone: 8,
        loop_restart: None,
        tones: &[
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
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
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 184,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 174,
                duration: 8,
                volume: 0,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 195,
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
        ],
    },
    Sfx {
        frames_per_tone: 8,
        loop_restart: None,
        tones: &[
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
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
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 220,
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
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 246,
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
        ],
    },
    Sfx {
        frames_per_tone: 8,
        loop_restart: None,
        tones: &[
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
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
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
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
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
                duration: 8,
                volume: 71,
                flags: 2,
            },
            Tone {
                frequency: 293,
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
        ],
    },
];

const MUSIC_DATA: &[Pattern] = &[
    Pattern {
        loop_start: false,
        loop_back: false,
        stop_at_end: false,
        sfxes: &[&SFX_DATA[0]],
    },
    Pattern {
        loop_start: false,
        loop_back: false,
        stop_at_end: false,
        sfxes: &[&SFX_DATA[4]],
    },
    Pattern {
        loop_start: false,
        loop_back: false,
        stop_at_end: false,
        sfxes: &[&SFX_DATA[0]],
    },
    Pattern {
        loop_start: false,
        loop_back: false,
        stop_at_end: true,
        sfxes: &[&SFX_DATA[8]],
    },
];

//endregion SFX and music data

const SFX0: *mut SfxPlayback = 0x19a0 as *mut SfxPlayback;
const SFX1: *mut SfxPlayback = 0x19b0 as *mut SfxPlayback;
const SFX2: *mut SfxPlayback = 0x19c0 as *mut SfxPlayback;
const SFX3: *mut SfxPlayback = 0x19d0 as *mut SfxPlayback;
const SFX_PLAYBACKS: *mut [&mut SfxPlayback; 4] = 0x19e0 as *mut [&mut SfxPlayback; 4];
const MUSIC_PLAYBACK: *mut MusicPlayback = 0x2000 as *mut MusicPlayback;

#[no_mangle]
fn start() {
    unsafe {
        *SFX0 = SfxPlayback::default();
        *SFX1 = SfxPlayback::default();
        *SFX2 = SfxPlayback::default();
        *SFX3 = SfxPlayback::default();
        let sfx_playbacks = &mut *SFX_PLAYBACKS;
        sfx_playbacks[0] = &mut *SFX0;
        sfx_playbacks[1] = &mut *SFX1;
        sfx_playbacks[2] = &mut *SFX2;
        sfx_playbacks[3] = &mut *SFX3;
        *MUSIC_PLAYBACK = MusicPlayback {
            playing: false,
            pattern_index: 0,
            patterns: MUSIC_DATA,
            sfx_playbacks,
        };
        (&mut *MUSIC_PLAYBACK).play(0);
    }
}

#[no_mangle]
fn update() {
    let msg = "Hello\nfrom\nRust!";

    let (w, h) = fmetrics(msg);

    unsafe { *DRAW_COLORS = 0x0320 }
    ftext(msg, 80 - (w as i32 / 2), 80 - (h as i32 / 2));

    unsafe {
        let music_playback = &mut *MUSIC_PLAYBACK;
        music_playback.update();
    }
}
