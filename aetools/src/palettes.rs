use image::{Pixel, Rgba};
use itertools::Itertools;

/// Default WASM-4 palette.
pub const WASM4_COLORS_ALPHA: [Rgba<u8>; 4] = [
    Rgba([0x07, 0x18, 0x21, 0xff]),
    Rgba([0x30, 0x68, 0x50, 0xff]),
    Rgba([0x86, 0xc0, 0x6c, 0xff]),
    Rgba([0xe0, 0xf8, 0xcf, 0xff]),
];

/// Fit an image palette of up to 4 greys to a target palette of 4 colors.
/// If the palette length is an exact match to the target palette length, we map the darkest grey to the darkest target color, etc., 1:1.
/// Otherwise, find the best fit between the palette and a target palette.
/// For example, if the input image uses only bright greys,
/// we want to map those lumas to bright colors in the target palette.
///
/// The input `palette` is assumed to be sorted in ascending order.
///
/// Returns a map of input palette indexes to output palette indexes.
/// Not all output palette entries may be used.
///
/// TODO: test this, it's messy
pub fn match_palette_to_target(palette: &Vec<u8>) -> Vec<u8> {
    assert!(palette.len() <= 4);

    if palette.len() == 4 {
        return (0..=3).into_iter().collect();
    }

    // Default WASM-4 palette converted to greyscale.
    let wasm4_greys: [u8; 4] = WASM4_COLORS_ALPHA.map(|rgba| rgba.to_luma().0[0]);

    let mut best_squared_error: Option<i32> = None;
    let mut best_palette_map: Option<Vec<u8>> = None;

    for candidate_palette_map in [0u8, 1, 2, 3].into_iter().combinations(palette.len()) {
        let mut candidate_palette: [Option<u8>; 4] = [None; 4];
        // Insert the undersized palette in order into
        // these indexes on a 4-entry palette.
        let mut p = 0usize;
        for i in &candidate_palette_map {
            candidate_palette[*i as usize] = Some(palette[p]);
            p += 1;
        }

        // Compare the palette to the target palette,
        // looking only at entries that the image will use.
        let squared_error: i32 = wasm4_greys
            .iter()
            .cloned()
            .zip(candidate_palette)
            .filter_map(|(w, maybe_c)| maybe_c.map(|c| w as i32 - c as i32))
            .map(|e| e * e)
            .sum();
        if best_squared_error.is_none() || squared_error < best_squared_error.unwrap() {
            best_squared_error = Some(squared_error);
            best_palette_map = Some(candidate_palette_map);
        }
    }

    best_palette_map.unwrap()
}
