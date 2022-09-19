use anyhow;
use deku::bitvec::*;
use deku::prelude::*;
use png;
use std::path::Path;

/// See https://youtu.be/aF1Yw_wu2cM?t=262
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
struct SpriteHeader {
    /// Width of sprite in 8-pixel tiles.
    #[deku(bits = "4")]
    w_tiles: u8,
    /// Height of sprite in 8-pixel tiles.
    #[deku(bits = "4")]
    h_tiles: u8,

    primary_buffer: PrimaryBuffer,
    initial_packet_type: InitialPacketType,
    // Followed by first bitplane data,
    // then `SecondBitplaneHeader`,
    // then second bitplane data.
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
struct SecondBitplaneHeader {
    encoding_method: EncodingMethod,
    initial_packet_type: InitialPacketType,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
enum PrimaryBuffer {
    #[deku(id = "0")]
    A,
    #[deku(id = "1")]
    B,
}

/// Packet types: see https://youtu.be/aF1Yw_wu2cM?t=585
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
enum InitialPacketType {
    #[deku(id = "0")]
    RLE,
    #[deku(id = "1")]
    Data,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
enum EncodingMethod {
    #[deku(id = "0")]
    Mode1,
    #[deku(id = "1")]
    Mode2Or3(EncodingMode2Or3),
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
enum EncodingMode2Or3 {
    #[deku(id = "0")]
    Mode2,
    #[deku(id = "1")]
    Mode3,
}

/// See https://youtu.be/aF1Yw_wu2cM?t=753
/// `n` may be in the range `1..=65536`.
fn encode_rle_count(n: u32) -> BitVec<Msb0, u8> {
    let u32_bits = u32::BITS as usize;

    // Number of bits in representation of n, minus 1
    let field_size = (u32_bits - n.leading_zeros() as usize) - 1;

    // See https://youtu.be/aF1Yw_wu2cM?t=789
    // All the bits in `n` after the first 1.
    let v = n & (u32::MAX >> (n.leading_zeros() + 1));

    // See https://youtu.be/aF1Yw_wu2cM?t=812
    // The next smallest power of 2, minus 2.
    // A string of 1s with a 0 terminator.
    let l = (n - v) - 2;

    let mut bv = bitvec![Msb0, u8; 0; 2 * field_size];
    bv[..field_size].store_be(l);
    bv[field_size..].store_be(v);
    bv
}

fn decode_rle_count(bv: BitVec<Msb0, u8>) -> u32 {
    let field_size = 1 + bv.first_zero().expect("Malformed RLE count length");

    let l: u32 = bv[..field_size].load_be();
    let v: u32 = bv[field_size..(2 * field_size)].load_be();
    l + v + 2
}

#[cfg(test)]
mod tests {
    use crate::pokepak::{decode_rle_count, encode_rle_count};
    use deku::bitvec::*;

    #[test]
    fn encode_2() {
        let expected = bitvec![Msb0, u8; 0, 0];
        let actual = encode_rle_count(2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn decode_2() {
        let expected = 2u32;
        let actual = decode_rle_count(bitvec![Msb0, u8; 0, 0]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn encode_45() {
        let expected = bitvec![Msb0, u8; 1, 1, 1, 1, 0, 0, 1, 1, 0, 1];
        let actual = encode_rle_count(45);
        assert_eq!(actual, expected);
    }

    #[test]
    fn decode_45() {
        let expected = 45u32;
        let actual = decode_rle_count(bitvec![Msb0, u8; 1, 1, 1, 1, 0, 0, 1, 1, 0, 1]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn encode_63282() {
        let expected = bitvec![Msb0, u8;
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1,
            0
        ];
        let actual = encode_rle_count(63282);
        assert_eq!(actual, expected);
    }

    #[test]
    fn decode_63282() {
        let expected = 63282u32;
        let actual = decode_rle_count(bitvec![Msb0, u8;
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1,
            0
        ]);
        assert_eq!(actual, expected);
    }
}

pub fn encode(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    todo!();
}

pub fn decode(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    todo!();
}
