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
        1,   // !
        4,   // "
        9,   // #
        12,  // $
        17,  // %
        21,  // &
        22,  // '
        24,  // (
        26,  // )
        31,  // *
        34,  // +
        36,  // ,
        38,  // -
        39,  // .
        42,  // /
        45,  // 0
        47,  // 1
        50,  // 2
        53,  // 3
        56,  // 4
        59,  // 5
        62,  // 6
        65,  // 7
        68,  // 8
        71,  // 9
        72,  // :
        74,  // ;
        77,  // <
        79,  // =
        82,  // >
        85,  // ?
        88,  // @
        91,  // A
        94,  // B
        97,  // C
        100, // D
        103, // E
        106, // F
        109, // G
        112, // H
        115, // I
        118, // J
        121, // K
        124, // L
        127, // M
        130, // N
        133, // O
        136, // P
        139, // Q
        142, // R
        145, // S
        148, // T
        151, // U
        154, // V
        157, // W
        160, // X
        163, // Y
        166, // Z
        168, // [
        171, // \
        173, // ]
        176, // ^
        179, // _
        181, // `
        184, // a
        187, // b
        190, // c
        193, // d
        196, // e
        198, // f
        201, // g
        204, // h
        205, // i
        207, // j
        210, // k
        212, // l
        215, // m
        218, // n
        221, // o
        224, // p
        227, // q
        229, // r
        232, // s
        235, // t
        238, // u
        241, // v
        244, // w
        247, // x
        250, // y
        253, // z
        256, // {
        257, // |
        260, // }
        265, // ~
    ],
});
