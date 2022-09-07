include!(concat!(env!("OUT_DIR"), "/assets.rs"));

#[cfg(feature = "buddy-alloc")]
mod alloc;
mod audio;
mod audio_data;
mod font;
mod gfx;
mod gfx_data;
mod intro;
mod map_data;
mod walkaround;
mod wasm4;

enum Mode {
    Intro,
    Walkaround,
}

static mut MODE: Mode = Mode::Intro;

#[no_mangle]
fn start() {
    audio::init();
    audio::music(0);
}

#[no_mangle]
fn update() {
    audio::music_update();

    unsafe {
        match MODE {
            Mode::Intro => {
                let continue_intro = intro::update();
                if !continue_intro {
                    MODE = Mode::Walkaround;
                }
            }
            Mode::Walkaround => walkaround::update(),
        }
    }
}
