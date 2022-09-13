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

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
enum Packet {
    #[deku(id = "0b00")]
    RLE,
    #[deku(id = "1")]
    Mode3,
}

/// Packet types: see https://youtu.be/aF1Yw_wu2cM?t=585

/// See https://youtu.be/aF1Yw_wu2cM?t=753
/// `n` may be in the range `1..=65536`.
fn write_rle_count(n: u32) -> BitVec {
    // Number of bits in representation of n, minus 1
    let field_size = (u32::BITS - n.leading_zeros()) as usize - 1;

    // See https://youtu.be/aF1Yw_wu2cM?t=789
    // All the bits in `n` after the first 1.
    let v = n & (u32::MAX >> (n.leading_zeros() + 1));

    // See https://youtu.be/aF1Yw_wu2cM?t=812
    // The next smallest power of 2, minus 2.
    // A string of 1s with a 0 terminator.
    let l = (n - v) - 2;

    let mut bv = bitvec![];
    bv.extend(&2u8.view_bits::<Lsb0>()[..2]);
    // bv.extend(&l.view_bits::<Lsb0>()[..field_size]);
    // bv.extend(&v.view_bits::<Lsb0>()[..field_size]);
    bv
}

#[cfg(test)]
mod tests {
    use crate::pokepak::write_rle_count;
    use deku::bitvec::*;

    #[test]
    fn encode_45() {
        let expected = bitvec![1, 1, 1, 1, 0, 0, 1, 1, 0, 1];
        let actual = write_rle_count(45);
        assert_eq!(actual, expected);
    }
}

pub fn encode(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    todo!();
}

pub fn decode(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    todo!();
}
