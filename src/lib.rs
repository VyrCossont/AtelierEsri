mod alchemy;
#[cfg(feature = "buddy-alloc")]
mod alloc;
mod asset_data;
mod audio;
mod audio_data;
mod font;
mod font_data;
mod gfx;
mod gfx_data;
mod intro;
mod map_data;
mod walkaround;
mod wasm4;

enum Mode {
    // Intro,
    // Walkaround,
    Alchemy,
}

static mut MODE: Mode = Mode::Alchemy;

#[no_mangle]
fn start() {
    // audio::init();
    // audio::music(0);
    // intro::init();
    alchemy::init();
}

#[no_mangle]
fn update() {
    // audio::music_update();

    unsafe {
        match MODE {
            // Mode::Intro => {
            //     let continue_intro = intro::update();
            //     if !continue_intro {
            //         MODE = Mode::Walkaround;
            //     }
            // }
            // Mode::Walkaround => walkaround::update(),
            Mode::Alchemy => alchemy::update(),
        }
    }
}
