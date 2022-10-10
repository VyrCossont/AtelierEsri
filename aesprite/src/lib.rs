/// Parameterized so we can use owned storage in tools and unowned storage in game.
pub struct Unisprite<T: UnispriteData> {
    pub w: i32,
    pub h: i32,
    /// 2 bits per pixel, no scanline padding.
    pub luma: T,
    /// 1 bit per pixel, no scanline padding.
    pub alpha: T,
}

pub trait UnispriteData {
    fn get(&self, i: usize) -> u8;
}

impl UnispriteData for &[u8] {
    fn get(&self, i: usize) -> u8 {
        self[i]
    }
}

#[cfg(feature = "std")]
impl UnispriteData for std::vec::Vec<u8> {
    fn get(&self, i: usize) -> u8 {
        self[i]
    }
}
