use crate::gfx::Lo5SplitSprite;
use crate::gfx_data::{
    BAR_COPPER, CURSOR_POINT, CURSOR_TOUCH, ORE_COPPER, RUBY, SAND, SCROLL, WOOD,
};
use crate::input;

static mut CLOCK: f64 = 0.0;

pub fn init() {}

const ITEMS: &[&Lo5SplitSprite] = &[&BAR_COPPER, &ORE_COPPER, &RUBY, &SAND, &SCROLL, &WOOD];

pub fn update() {
    let (mx, my) = input::mouse().pos;
    let clock = unsafe { CLOCK };
    let mut touching = false;
    for (i, item) in ITEMS.iter().enumerate() {
        let clock_i = clock + i as f64 * 2.0 * std::f64::consts::PI / ITEMS.len() as f64;
        let x = (80.0 - 8.0 + clock_i.cos() * 24.0) as i32;
        let y = (80.0 - 8.0 + clock_i.sin() * 24.0) as i32;
        item.blit(x, y, 0);
        if mx >= x && mx < x + 16 && my >= y && my < y + 16 {
            touching = true;
        }
        let x2 = (80.0 - 16.0 + -clock_i.cos() * 56.0) as i32;
        let y2 = (80.0 - 16.0 + clock_i.sin() * 56.0) as i32;
        item.blit2x(x2, y2);
        if mx >= x2 && mx < x2 + 32 && my >= y2 && my < y2 + 32 {
            touching = true;
        }
    }
    unsafe {
        CLOCK += 0.01;
    }

    let cursor = if touching { CURSOR_TOUCH } else { CURSOR_POINT };
    cursor.draw((mx, my));
}
