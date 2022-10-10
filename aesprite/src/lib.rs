use std::ops::Index;

/// Parameterized so we can use owned storage in tools and unowned storage in game.
pub struct Unisprite<U: Index<usize, Output = u8>> {
    pub w: i32,
    pub h: i32,
    /// 2 bits per pixel, no scanline padding.
    pub luma: U,
    /// 1 bit per pixel, no scanline padding.
    pub alpha: U,
}
