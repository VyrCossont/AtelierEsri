include!(concat!(env!("OUT_DIR"), "/assets.rs"));

#[cfg(feature = "buddy-alloc")]
mod alloc;
mod font;
mod wasm4;
use font::*;
use wasm4::*;

#[no_mangle]
fn update() {
    let msg = "Hello\nfrom\nRust!";

    let (w, h) = fmetrics(msg);

    unsafe { *DRAW_COLORS = 0x0320 }
    ftext(msg, 80 - (w as i32 / 2), 80 - (h as i32 / 2));
}
