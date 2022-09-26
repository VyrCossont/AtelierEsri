use crate::gfx::Lo5SplitSprite;
use crate::gfx_data::{BAR_COPPER, ORE_COPPER, RUBY, SAND, SCROLL, WOOD};

static mut CLOCK: f64 = 0.0;

pub fn init() {}

const ITEMS: &[&Lo5SplitSprite] = &[&BAR_COPPER, &ORE_COPPER, &RUBY, &SAND, &SCROLL, &WOOD];

pub fn update() {
    let clock = unsafe { CLOCK };
    for (i, item) in ITEMS.iter().enumerate() {
        let clock_i = clock + i as f64 * 2.0 * std::f64::consts::PI / ITEMS.len() as f64;
        let x = 80.0 - 8.0 + clock_i.cos() * 24.0;
        let y = 80.0 - 8.0 + clock_i.sin() * 24.0;
        item.blit(x as i32, y as i32, 0);
        let x2 = 80.0 - 16.0 + -clock_i.cos() * 56.0;
        let y2 = 80.0 - 16.0 + clock_i.sin() * 56.0;
        item.blit2x(x2 as i32, y2 as i32);
    }
    unsafe {
        CLOCK += 0.03;
    }
}
