use crate::asset_data;
use crate::font::{Font, ProportionalFont};

pub const TINY: Font = Font::Proportional(ProportionalFont {
    image_data: &asset_data::TINY_FONT,
    image_width: asset_data::TINY_FONT_WIDTH,
    image_height: asset_data::TINY_FONT_HEIGHT,
    image_flags: asset_data::TINY_FONT_FLAGS,
    space_width: 3,
    kerning: 1,
    line_spacing: 1,
    src_xs: &[
        0,   // !
        1,   // "
        4,   // #
        9,   // $
        12,  // %
        17,  // &
        21,  // '
        22,  // (
        24,  // )
        26,  // *
        31,  // +
        34,  // ,
        36,  // -
        38,  // .
        39,  // /
        42,  // 0
        45,  // 1
        47,  // 2
        50,  // 3
        53,  // 4
        56,  // 5
        59,  // 6
        62,  // 7
        65,  // 8
        68,  // 9
        71,  // :
        72,  // ;
        74,  // <
        77,  // =
        79,  // >
        82,  // ?
        85,  // @
        88,  // A
        91,  // B
        94,  // C
        97,  // D
        100, // E
        103, // F
        106, // G
        109, // H
        112, // I
        115, // J
        118, // K
        121, // L
        124, // M
        127, // N
        130, // O
        133, // P
        136, // Q
        139, // R
        142, // S
        145, // T
        148, // U
        151, // V
        154, // W
        157, // X
        160, // Y
        163, // Z
        166, // [
        168, // \
        171, // ]
        173, // ^
        176, // _
        179, // `
        181, // a
        184, // b
        187, // c
        190, // d
        193, // e
        196, // f
        198, // g
        201, // h
        204, // i
        205, // j
        207, // k
        210, // l
        212, // m
        215, // n
        218, // o
        221, // p
        224, // q
        227, // r
        229, // s
        232, // t
        235, // u
        238, // v
        241, // w
        244, // x
        247, // y
        250, // z
        253, // {
        256, // |
        257, // }
        260, // ~
        265, // DEL (not used)
    ],
});
