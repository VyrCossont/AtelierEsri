use crate::wasm4;
use std::cmp::{max, min};

// region split sprites

/// Logical sprite composed of 2 other sprites,
/// the first 2BPP sprite with transparent color, color 0, color 1, color 2,
/// the second a 1BPP sprite with transparent color, color 3,
/// letting us draw a 4-color sprite with transparency.
/// Use `tools lo5` to split those PNGs from an input PNG.
pub struct Lo5SplitSprite<'a> {
    pub w: u32,
    pub h: u32,
    pub lo4: &'a [u8],
    pub hi2: &'a [u8],
}

impl Lo5SplitSprite<'_> {
    pub fn blit_sub(
        &self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        src_x: u32,
        src_y: u32,
        flags: u32,
    ) {
        // TODO: standardize which color indexes are the brighter ones across the project.
        unsafe { *wasm4::DRAW_COLORS = 0x2340 }
        wasm4::blit_sub(
            self.lo4,
            x,
            y,
            width,
            height,
            src_x,
            src_y,
            self.w,
            flags | wasm4::BLIT_2BPP,
        );
        unsafe { *wasm4::DRAW_COLORS = 0x0010 }
        wasm4::blit_sub(
            self.hi2,
            x,
            y,
            width,
            height,
            src_x,
            src_y,
            self.w,
            flags | wasm4::BLIT_1BPP,
        );
    }

    pub fn blit(&self, x: i32, y: i32, flags: u32) {
        self.blit_sub(x, y, self.w, self.h, 0, 0, flags);
    }

    // TODO: this produces obvious edge artifacts on 45° diagonals.
    //  Try using a color + mask representation instead.
    pub fn blit2x(&self, x: i32, y: i32) {
        unsafe {
            SCALE_BUFFER.fill(0);
            scale2x(self.lo4, self.w, self.h, 2, SCALE_BUFFER);
            unsafe { *wasm4::DRAW_COLORS = 0x2340 }
            wasm4::blit(SCALE_BUFFER, x, y, self.w * 2, self.h * 2, wasm4::BLIT_2BPP);
            SCALE_BUFFER.fill(0);
            scale2x(self.hi2, self.w, self.h, 1, SCALE_BUFFER);
            unsafe { *wasm4::DRAW_COLORS = 0x0010 }
            wasm4::blit(SCALE_BUFFER, x, y, self.w * 2, self.h * 2, wasm4::BLIT_1BPP);
        }
    }
}

// endregion split sprites

// region sprite scaling

static mut SCALE_BUFFER: &mut [u8] = &mut [0; (64 * 3 * 64 * 3) / 4];

fn pixel_offset(w: u32, bit_depth: u32, x: u32, y: u32) -> (usize, u8, u8) {
    let bit_offset = bit_depth * (w * y + x);
    let byte_offset = (bit_offset / u8::BITS) as usize;
    let shift = ((u8::BITS - bit_depth) - bit_offset % u8::BITS) as u8;
    let mask: u8 = if bit_depth == 2 { 0b11 } else { 0b1 };
    (byte_offset, shift, mask)
}

fn get_pixel(data: &[u8], w: u32, bit_depth: u32, x: u32, y: u32) -> u8 {
    let (byte_offset, shift, mask) = pixel_offset(w, bit_depth, x, y);
    (data[byte_offset] >> shift) & mask
}

fn set_pixel(data: &mut [u8], w: u32, bit_depth: u32, x: u32, y: u32, pixel: u8) {
    let (byte_offset, shift, mask) = pixel_offset(w, bit_depth, x, y);
    data[byte_offset] |= (pixel & mask) << shift;
}

/// See https://en.wikipedia.org/wiki/Pixel-art_scaling_algorithms#EPX/Scale2%C3%97/AdvMAME2%C3%97
fn scale2x(data: &[u8], w: u32, h: u32, bit_depth: u32, scale_buffer: &mut [u8]) {
    let buf_num_bytes = 1 + pixel_offset(2 * w, bit_depth, 2 * w, 2 * h).0;
    if buf_num_bytes >= scale_buffer.len() {
        wasm4::trace("Ran out of room in the scale buffer");
        panic!("Ran out of room in the scale buffer");
    }
    for y in 0..h {
        for x in 0..w {
            let p = get_pixel(data, w, bit_depth, x, y);
            let a = if y == 0 {
                0
            } else {
                get_pixel(data, w, bit_depth, x, y - 1)
            };
            let b = if x == w - 1 {
                0
            } else {
                get_pixel(data, w, bit_depth, x + 1, y)
            };
            let c = if x == 0 {
                0
            } else {
                get_pixel(data, w, bit_depth, x - 1, y)
            };
            let d = if y == h - 1 {
                0
            } else {
                get_pixel(data, w, bit_depth, x, y + 1)
            };
            let p1 = if c == a && c != d && a != b { a } else { p };
            let p2 = if a == b && a != c && b != d { b } else { p };
            let p3 = if d == c && d != b && c != a { c } else { p };
            let p4 = if b == d && b != a && d != c { d } else { p };
            set_pixel(scale_buffer, 2 * w, bit_depth, 2 * x, 2 * y, p1);
            set_pixel(scale_buffer, 2 * w, bit_depth, 2 * x + 1, 2 * y, p2);
            set_pixel(scale_buffer, 2 * w, bit_depth, 2 * x, 2 * y + 1, p3);
            set_pixel(scale_buffer, 2 * w, bit_depth, 2 * x + 1, 2 * y + 1, p4);
        }
    }
}

// endregion sprite scaling

// region character sprites

#[repr(u8)]
#[derive(Default, Copy, Clone)]
pub enum Orientation {
    E = 0,
    NE = 1,
    N = 2,
    NW = 3,
    W = 4,
    SW = 5,
    #[default]
    S = 6,
    SE = 7,
}

impl From<(i32, i32)> for Orientation {
    fn from((x, y): (i32, i32)) -> Self {
        match (x, y) {
            _ if x > 0 && y == 0 => Orientation::E,
            _ if x > 0 && y < 0 => Orientation::NE,
            _ if x == 0 && y < 0 => Orientation::N,
            _ if x < 0 && y < 0 => Orientation::NW,
            _ if x < 0 && y == 0 => Orientation::W,
            _ if x < 0 && y > 0 => Orientation::SW,
            _ if x == 0 && y > 0 => Orientation::S,
            _ if x > 0 && y > 0 => Orientation::SE,
            _ => Orientation::default(),
        }
    }
}

/// Assumed to use a sprite strip.
pub struct CharacterSprite<'a> {
    pub image_w: u32,
    pub image_h: u32,
    pub image: &'a [u8],
    pub draw_colors: u16,
    pub sprite_w: u32,
    pub walk_cycle_length: usize,
    pub orientation_starts_flags: [(usize, u32); 8],
}

impl CharacterSprite<'_> {
    pub fn draw(&self, x: i32, y: i32, w: usize, o: Orientation) {
        let (start, flags) = self.orientation_starts_flags[o as usize];
        let sprite_num: usize = start + (w % self.walk_cycle_length);
        unsafe { *wasm4::DRAW_COLORS = self.draw_colors };
        wasm4::blit_sub(
            self.image,
            x,
            y,
            self.sprite_w,
            self.image_h,
            sprite_num as u32 * self.sprite_w,
            0,
            self.image_w,
            flags,
        );
    }
}

// endregion character sprites

// region map

type TileId = u8;

/// Map tileset. Wrapper around an image.
pub struct Tileset<'a> {
    pub tile_width: u32,
    pub tile_height: u32,
    pub image: &'a Lo5SplitSprite<'a>,
}

impl Tileset<'_> {
    pub fn blit(&self, tile: TileId, x: i32, y: i32) {
        if tile == 0 {
            return;
        }

        let tiles_per_row = self.image.w / self.tile_width;
        let row = (tile as u32 - 1) / tiles_per_row;
        let col = (tile as u32 - 1) % tiles_per_row;
        self.image.blit_sub(
            x,
            y,
            self.tile_width,
            self.tile_height,
            col * self.tile_width,
            row * self.tile_height,
            0,
        );
    }
}

/// Map layer. Someday we'll support more than one.
pub struct Layer<'a> {
    pub width_tiles: u32,
    pub height_tiles: u32,
    pub tileset: &'a Tileset<'a>,
    pub tiles: &'a [TileId],
}

impl Layer<'_> {
    /// Return dimensions in pixels instead of tiles.
    pub fn dimensions(&self) -> (u32, u32) {
        (
            self.width_tiles * self.tileset.tile_width,
            self.height_tiles * self.tileset.tile_height,
        )
    }

    pub fn draw(&self, x: i32, y: i32, map_x: i32, map_y: i32, w: u32, h: u32) {
        let map_x_to_x = x - map_x;
        let map_y_to_y = y - map_y;

        let map_x_min = max(0, map_x - (map_x % self.tileset.tile_width as i32));
        let map_x_max = min(
            self.width_tiles as i32 * self.tileset.tile_width as i32,
            map_x + w as i32 + ((map_x + w as i32) % self.tileset.tile_width as i32),
        );

        let map_y_min = max(0, map_y - (map_y % self.tileset.tile_height as i32));
        let map_y_max = min(
            self.height_tiles as i32 * self.tileset.tile_height as i32,
            map_y + h as i32 + ((map_y + h as i32) % self.tileset.tile_height as i32),
        );

        for map_y_tile in (map_y_min..map_y_max).step_by(self.tileset.tile_height as usize) {
            for map_x_tile in (map_x_min..map_x_max).step_by(self.tileset.tile_width as usize) {
                let row = map_y_tile / self.tileset.tile_height as i32;
                let col = map_x_tile / self.tileset.tile_width as i32;
                let tile_index = row * self.width_tiles as i32 + col;
                let tile = self.tiles[tile_index as usize];
                self.tileset
                    .blit(tile, map_x_tile + map_x_to_x, map_y_tile + map_y_to_y);
            }
        }
    }
}

// endregion map

// region thick lines

pub fn thick_hline(x: i32, y: i32, len: u32, h: i32) {
    for dy in 0..h {
        wasm4::hline(x, y + dy, len);
    }
}

pub fn thick_vline(x: i32, y: i32, len: u32, w: i32) {
    for dx in 0..w {
        wasm4::vline(x + dx, y, len);
    }
}

pub fn thick_line(x1: i32, y1: i32, x2: i32, y2: i32, w: i32, h: i32) {
    for dy in 0..h {
        for dx in 0..w {
            wasm4::line(x1 + dx, y1 + dy, x2 + dx, y2 + dy);
        }
    }
}

// endregion thick lines
