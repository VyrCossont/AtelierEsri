/// https://pico-8.fandom.com/wiki/Palette#0..15:_Official_base_colors
pub const DEFAULT_PALETTE: [u32; 16] = [
    0x00000000, // black
    0x001d2b53, // dark blue
    0x007e2553, // dark purple
    0x00008751, // dark green
    0x00ab5236, // brown
    0x005f574f, // dark grey
    0x00c2c3c7, // light grey
    0x00fff1e8, // white
    0x00ff004d, // red
    0x00ffa300, // orange
    0x00ffec27, // yellow
    0x0000e436, // green
    0x0029adff, // blue
    0x0083769c, // lavender
    0x00ff77a8, // pink
    0x00ffccaa, // light peach
];

pub const ALT_PALETTE_OFFSET: u8 = 128;

/// https://pico-8.fandom.com/wiki/Palette#128..143:_Undocumented_extra_colors
pub const ALT_PALETTE: [u32; 16] = [
    0x00291814, // brownish black
    0x00111d35, // darker blue
    0x00422136, // darker purple
    0x00125359, // blue green
    0x00742f29, // dark brown
    0x0049333b, // darker grey
    0x00a28879, // medium grey
    0x00f3ef7d, // light yellow
    0x00be1250, // dark red
    0x00ff6c24, // dark orange
    0x00a8e72e, // lime green
    0x0000b543, // medium green
    0x00065ab5, // true blue
    0x00754665, // mauve
    0x00ff6e59, // dark peach
    0x00ff9d81, // peach
];
