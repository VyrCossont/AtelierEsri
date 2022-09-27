use crate::wasm4;

pub struct MouseState {
    pub pos: (i32, i32),
    pub left: bool,
    pub middle: bool,
    pub right: bool,
}

pub fn mouse() -> MouseState {
    let (x, y, buttons) = unsafe { (*wasm4::MOUSE_X, *wasm4::MOUSE_Y, *wasm4::MOUSE_BUTTONS) };
    MouseState {
        pos: (x as i32, y as i32),
        left: buttons & wasm4::MOUSE_LEFT != 0,
        middle: buttons & wasm4::MOUSE_MIDDLE != 0,
        right: buttons & wasm4::MOUSE_RIGHT != 0,
    }
}
