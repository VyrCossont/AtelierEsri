use crate::audio_data::MUSIC_DATA;
use crate::wasm4::tone;

pub struct Tone {
    pub frequency: u32,
    pub duration: u32,
    pub volume: u32,
    pub flags: u32,
}

pub struct Sfx<'a> {
    pub frames_per_tone: u8,
    pub loop_restart: Option<usize>,
    pub tones: &'a [Tone],
}

impl<'a> Sfx<'a> {
    pub fn looped(&self) -> bool {
        self.loop_restart.is_some()
    }
}

#[derive(Default)]
pub struct SfxPlayback<'a> {
    pub frame_counter: u8,
    pub tone_index: usize,
    pub sfx: Option<&'a Sfx<'a>>,
}

impl<'a> SfxPlayback<'a> {
    pub fn play(&mut self, sfx: &'a Sfx<'a>) {
        self.frame_counter = 0;
        self.tone_index = 0;
        self.sfx = Some(sfx);
    }

    pub fn stop(&mut self) {
        self.sfx = None;
    }

    pub fn playing(&self) -> bool {
        if let Some(sfx) = self.sfx {
            self.tone_index < sfx.tones.len()
        } else {
            false
        }
    }

    /// Returns whether it will continue to play.
    /// Always returns true for looped SFX.
    pub fn update(&mut self) -> bool {
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

pub struct Pattern<'a> {
    pub loop_start: bool,
    pub loop_back: bool,
    pub stop_at_end: bool,
    pub sfxes: &'a [&'a Sfx<'a>],
}

pub struct MusicPlayback<'a> {
    pub playing: bool,
    pub pattern_index: usize,
    pub patterns: &'a [Pattern<'a>],
    pub sfx_playbacks: &'a mut [&'a mut SfxPlayback<'a>],
}

impl MusicPlayback<'_> {
    /// Returns whether it will continue to play.
    /// Always returns true for looped music.
    /// See https://www.lexaloffle.com/dl/docs/pico-8_manual.html#Flow_control
    pub fn update(&mut self) -> bool {
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

    pub fn start_pattern(&mut self) {
        let sfxes = self.patterns[self.pattern_index].sfxes;
        for (sfx_playback, sfx) in self.sfx_playbacks.iter_mut().zip(sfxes) {
            sfx_playback.play(sfx);
        }
    }

    pub fn play(&mut self, pattern_index: usize) {
        self.stop();
        self.pattern_index = pattern_index;
        self.playing = true;
        self.start_pattern();
    }

    pub fn stop(&mut self) {
        self.playing = false;
        for sfx_playback in self.sfx_playbacks.iter_mut() {
            sfx_playback.stop();
        }
    }
}

// TODO: create mutable globals with correct sizes and compiler-assigned addresses
const SFX0: *mut SfxPlayback = 0x19a0 as *mut SfxPlayback;
const SFX1: *mut SfxPlayback = 0x19b0 as *mut SfxPlayback;
const SFX2: *mut SfxPlayback = 0x19c0 as *mut SfxPlayback;
const SFX3: *mut SfxPlayback = 0x19d0 as *mut SfxPlayback;
const SFX_PLAYBACKS: *mut [&mut SfxPlayback; 4] = 0x19e0 as *mut [&mut SfxPlayback; 4];
const MUSIC_PLAYBACK: *mut MusicPlayback = 0x2000 as *mut MusicPlayback;

pub fn init() {
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
    }
}

pub fn music(pattern_index: usize) {
    unsafe {
        (&mut *MUSIC_PLAYBACK).play(pattern_index);
    }
}

pub fn music_update() {
    unsafe {
        (&mut *MUSIC_PLAYBACK).update();
    }
}
