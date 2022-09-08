use crate::wasm4;
use std::cmp::{max, min};

// region split sprites

/// Sprite composed of 2 2BPP sprites,
/// the 1st with transparent color, color 0, color 1, unused color,
/// the 2nd with transparent color, color 2, color 3, unused color,
/// letting us draw a 4-color sprite with transparency.
/// TODO: use 1 2BPP sprite for transparent, color 0, color 1, color 2
///     and 1 1BPP sprite for transparent, color 3?
pub struct SplitSprite<'a> {
    pub w: u32,
    pub h: u32,
    pub layers: [&'a [u8]; 2],
}

impl SplitSprite<'_> {
    pub fn blit(&self, x: i32, y: i32, flags: u32) {
        // TODO: standardize which color indexes are the brighter ones across the project.
        unsafe { *wasm4::DRAW_COLORS = 0x0043 }
        wasm4::blit(
            self.layers[0],
            x,
            y,
            self.w,
            self.h,
            flags | wasm4::BLIT_2BPP,
        );
        unsafe { *wasm4::DRAW_COLORS = 0x0021 }
        wasm4::blit(
            self.layers[1],
            x,
            y,
            self.w,
            self.h,
            flags | wasm4::BLIT_2BPP,
        );
    }
}

// endregion split sprites

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
    pub image_width: u32,
    pub image: &'a [u8],
    pub image_flags: u32,
}

impl Tileset<'_> {
    pub fn blit(&self, tile: TileId, x: i32, y: i32) {
        if tile == 0 {
            return;
        }

        let tiles_per_row = self.image_width / self.tile_width;
        let row = (tile as u32 - 1) / tiles_per_row;
        let col = (tile as u32 - 1) % tiles_per_row;
        wasm4::blit_sub(
            self.image,
            x,
            y,
            self.tile_width,
            self.tile_height,
            col * self.tile_width,
            row * self.tile_height,
            self.image_width,
            self.image_flags,
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
