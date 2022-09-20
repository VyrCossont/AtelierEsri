use anyhow;
use bitvec::mem::BitMemory;
use deku::bitvec::*;
use deku::prelude::*;
use png;
use std::ops::Not;
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
    initial_packet_type: PacketType,
    // Followed by first bitplane data,
    // then `SecondBitplaneHeader`,
    // then second bitplane data.
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
struct SecondBitplaneHeader {
    encoding_method: EncodingMethod,
    initial_packet_type: PacketType,
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
/// and https://youtu.be/aF1Yw_wu2cM?t=981
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
enum PacketType {
    #[deku(id = "0")]
    RLE,
    #[deku(id = "1")]
    Data,
}

impl Not for PacketType {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::RLE => Self::Data,
            Self::Data => Self::RLE,
        }
    }
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

impl EncodingMethod {
    const MODE_1: Self = Self::Mode1;
    const MODE_2: Self = Self::Mode2Or3(EncodingMode2Or3::Mode2);
    const MODE_3: Self = Self::Mode2Or3(EncodingMode2Or3::Mode3);
}

struct BitReader<'a> {
    bits: &'a BitSlice<Msb0, u8>,
    pos: usize,
}

impl BitReader<'_> {
    fn new<'a>(bits: &'a BitSlice<Msb0, u8>) -> BitReader<'a> {
        BitReader { bits, pos: 0 }
    }

    fn read(&mut self, num_bits: usize) -> anyhow::Result<&BitSlice<Msb0, u8>> {
        if self.pos + num_bits > self.bits.len() {
            anyhow::bail!(
                "Ran out of bits: len = {}, requested pos end = {}",
                self.bits.len(),
                self.pos + num_bits
            );
        }
        let r = &self.bits[self.pos..self.pos + num_bits];
        self.pos += num_bits;
        Ok(r)
    }

    fn read_until_zero(&mut self) -> anyhow::Result<&BitSlice<Msb0, u8>> {
        if let Some(first_zero) = self.bits.first_zero() {
            self.read(1 + first_zero)
        } else {
            anyhow::bail!(
                "Couldn't find a zero before end of bits: len = {}",
                self.bits.len(),
            );
        }
    }

    fn load_be<M>(&mut self, num_bits: usize) -> anyhow::Result<M>
    where
        M: BitMemory,
    {
        Ok(self.read(num_bits)?.load_be())
    }
}

struct BitWriter {
    bits: BitVec<Msb0, u8>,
}

impl BitWriter {
    fn new() -> Self {
        BitWriter {
            bits: bitvec![Msb0, u8; 0; 0],
        }
    }

    fn write(&mut self, bits: &BitSlice<Msb0, u8>) {
        self.bits.extend(bits);
    }

    fn store_be<M>(&mut self, n: usize, val: M)
    where
        M: BitMemory,
    {
        let mut bits = bitvec![Msb0, u8; 0; n];
        bits.store_be(val);
        self.write(&bits);
    }
}

/// See https://youtu.be/aF1Yw_wu2cM?t=753
/// and https://youtu.be/aF1Yw_wu2cM?t=932
/// for the plus-one bit.
/// `n` may be in the range `1..=65536`.
fn write_rle_count(n: u32, writer: &mut BitWriter) {
    let n_plus_one = n + 1;
    let u32_bits = u32::BITS as usize;

    // Number of bits in representation of n, minus 1
    let field_size = (u32_bits - n_plus_one.leading_zeros() as usize) - 1;

    // See https://youtu.be/aF1Yw_wu2cM?t=789
    // All the bits in `n_plus_one` after the first 1.
    let v = n_plus_one & (u32::MAX >> (n_plus_one.leading_zeros() + 1));

    // See https://youtu.be/aF1Yw_wu2cM?t=812
    // The next smallest power of 2, minus 2.
    // A string of 1s with a 0 terminator.
    let l = (n_plus_one - v) - 2;

    writer.store_be(field_size, l);
    writer.store_be(field_size, v);
}

fn read_rle_count(reader: &mut BitReader) -> anyhow::Result<u32> {
    let l_bits = reader.read_until_zero()?;
    let l: u32 = l_bits.load_be();
    let field_size = l_bits.len();
    let v: u32 = reader.load_be(field_size)?;
    let n_plus_one = l + v + 2;
    Ok(n_plus_one - 1)
}

fn decompress_bitplane(
    w_tiles: u8,
    h_tiles: u8,
    initial_packet_type: PacketType,
    bv: BitVec<Msb0, u8>,
) -> BitVec<Msb0, u8> {
    let mut buf = bitvec![Msb0, u8; 0; w_tiles as usize * 8 * h_tiles as usize * 8];
    let mut packet_type = initial_packet_type;
    let mut buf_pos = 0usize;
    while buf_pos < buf.len() {
        match packet_type {
            PacketType::RLE => decode_rle_packet(),
            PacketType::Data => decode_data_packet(),
        }
        packet_type = !packet_type;
    }
    todo!()
}

fn decode_rle_packet() {
    todo!()
}

fn decode_data_packet() {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::pokepak::{read_rle_count, write_rle_count, BitReader, BitWriter, EncodingMethod};
    use deku::bitvec::*;
    use deku::prelude::*;

    fn encode_rle_count(n: u32) -> BitVec<Msb0, u8> {
        let mut writer = BitWriter::new();
        write_rle_count(n, &mut writer);
        writer.bits
    }

    #[test]
    fn encode_rle_count_1() {
        let expected = bitvec![Msb0, u8; 0, 0];
        let actual = encode_rle_count(1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn encode_rle_count_44() {
        let expected = bitvec![Msb0, u8; 1, 1, 1, 1, 0, 0, 1, 1, 0, 1];
        let actual = encode_rle_count(44);
        assert_eq!(actual, expected);
    }

    #[test]
    fn encode_rle_count_63281() {
        let expected = bitvec![Msb0, u8;
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1,
            0
        ];
        let actual = encode_rle_count(63281);
        assert_eq!(actual, expected);
    }

    fn decode_rle_count(bv: BitVec<Msb0, u8>) -> u32 {
        let mut reader = BitReader::new(&bv);
        read_rle_count(&mut reader).unwrap()
    }

    #[test]
    fn decode_rle_count_1() {
        let expected = 1u32;
        let actual = decode_rle_count(bitvec![Msb0, u8; 0, 0]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn decode_rle_count_44() {
        let expected = 44u32;
        let actual = decode_rle_count(bitvec![Msb0, u8; 1, 1, 1, 1, 0, 0, 1, 1, 0, 1]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn decode_rle_count_63281() {
        let expected = 63281u32;
        let actual = decode_rle_count(bitvec![Msb0, u8;
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1,
            0
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn encode_mode1() {
        let expected = bitvec![Msb0, u8; 0];
        let mut actual = bitvec![Msb0, u8; 0; 0];
        EncodingMethod::MODE_1.write(&mut actual, ()).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn encode_mode2() {
        let expected = bitvec![Msb0, u8; 1, 0];
        let mut actual = bitvec![Msb0, u8; 0; 0];
        EncodingMethod::MODE_2.write(&mut actual, ()).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn encode_mode3() {
        let expected = bitvec![Msb0, u8; 1, 1];
        let mut actual = bitvec![Msb0, u8; 0; 0];
        EncodingMethod::MODE_3.write(&mut actual, ()).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn decode_mode1() {
        let expected = EncodingMethod::MODE_1;
        let actual = EncodingMethod::read(&bitvec![Msb0, u8; 0], ()).unwrap().1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn decode_mode2() {
        let expected = EncodingMethod::MODE_2;
        let actual = EncodingMethod::read(&bitvec![Msb0, u8; 1, 0], ())
            .unwrap()
            .1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn decode_mode3() {
        let expected = EncodingMethod::MODE_3;
        let actual = EncodingMethod::read(&bitvec![Msb0, u8; 1, 1], ())
            .unwrap()
            .1;
        assert_eq!(actual, expected);
    }
}

pub fn encode(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    todo!();
}

pub fn decode(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    todo!();
}
