//! Implements Pokémon sprite compression algorithm
//! described by https://youtu.be/ZI50XUeN6QE
//! and https://youtu.be/aF1Yw_wu2cM.
//! TODO: confirm that typical compression ratio is around 1.5:1.
//! TODO: implement delta coding
//! TODO: implement bitplane XOR modes
//! TODO: implement PNG import/export.

use anyhow;
use bitvec::mem::BitMemory;
use deku::bitvec::*;
use deku::prelude::*;
use png::{BitDepth, ColorType, Decoder, Encoder};
use std::fs;
use std::fs::File;
use std::io::BufWriter;
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
    // Followed by first bitplane data,
    // then `SecondBitplaneHeader`,
    // then second bitplane data.
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
struct SecondBitplaneHeader {
    encoding_method: EncodingMethod,
}

/// See https://youtu.be/aF1Yw_wu2cM?t=1352
/// Controls which bitplane goes first and thus modifies which one the encoding methods modify most.
#[derive(Debug, PartialEq, DekuRead, DekuWrite, Copy, Clone)]
#[deku(type = "u8", bits = "1")]
enum PrimaryBuffer {
    #[deku(id = "0")]
    MostSignificantBitplane,
    #[deku(id = "1")]
    LeastSignificantBitplane,
}

/// See https://youtu.be/aF1Yw_wu2cM?t=585
/// and https://youtu.be/aF1Yw_wu2cM?t=981
#[derive(Debug, PartialEq, DekuRead, DekuWrite, Copy, Clone)]
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

/// See https://youtu.be/aF1Yw_wu2cM?t=1331
#[derive(Debug, PartialEq, DekuRead, DekuWrite, Copy, Clone)]
#[deku(type = "u8", bits = "1")]
enum EncodingMethod {
    #[deku(id = "0")]
    Mode1,
    #[deku(id = "1")]
    Mode2Or3(EncodingMode2Or3),
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Copy, Clone)]
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

    fn delta_encode_bp1(&self) -> bool {
        match self {
            Self::Mode1 => true,
            Self::Mode2Or3(EncodingMode2Or3::Mode2) => false,
            Self::Mode2Or3(EncodingMode2Or3::Mode3) => true,
        }
    }

    fn xor_bp1_with_bp0(&self) -> bool {
        match self {
            Self::Mode1 => false,
            Self::Mode2Or3(EncodingMode2Or3::Mode2) => true,
            Self::Mode2Or3(EncodingMode2Or3::Mode3) => true,
        }
    }
}

struct BitReader<'a> {
    bits: &'a BitSlice<Msb0, u8>,
    pos: usize,
}

impl BitReader<'_> {
    fn new(bits: &BitSlice<Msb0, u8>) -> BitReader {
        BitReader { bits, pos: 0 }
    }

    fn len(&self) -> usize {
        self.bits.len()
    }

    fn read(&mut self, num_bits: usize) -> anyhow::Result<&BitSlice<Msb0, u8>> {
        if self.pos + num_bits > self.len() {
            anyhow::bail!(
                "Ran out of bits: len = {}, requested pos end = {}",
                self.len(),
                self.pos + num_bits
            );
        }
        let r = &self.bits[self.pos..self.pos + num_bits];
        self.pos += num_bits;
        Ok(r)
    }

    fn read_until_zero(&mut self) -> anyhow::Result<&BitSlice<Msb0, u8>> {
        if let Some(first_zero) = self.bits[self.pos..].first_zero() {
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

    fn deku_read<'a, T>(&'a mut self) -> anyhow::Result<T>
    where
        T: DekuRead<'a>,
    {
        let slice = &self.bits[self.pos..];
        let (rest, val) = T::read(slice, ())?;
        self.pos += slice.len() - rest.len();
        Ok(val)
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

    fn len(&self) -> usize {
        self.bits.len()
    }

    fn write(&mut self, bits: &BitSlice<Msb0, u8>) {
        self.bits.extend(bits);
    }

    fn extend_with_zeroes(&mut self, num_bits: usize) {
        let new_len = self.len() + num_bits;
        self.bits.resize(new_len, false);
    }

    fn store_be<M>(&mut self, n: usize, val: M)
    where
        M: BitMemory,
    {
        let mut bits = bitvec![Msb0, u8; 0; n];
        bits.store_be(val);
        self.write(&bits);
    }

    fn deku_write<T>(&mut self, val: T) -> anyhow::Result<()>
    where
        T: DekuWrite,
    {
        val.write(&mut self.bits, ())
            .map_err(|e| anyhow::anyhow!(e))
    }
}

/// See https://youtu.be/aF1Yw_wu2cM?t=753
/// and https://youtu.be/aF1Yw_wu2cM?t=932
/// for the plus-one bit.
/// `n` may be in the range `1..=65536`.
fn write_rle_count(n: u32, writer: &mut BitWriter) {
    assert!(n > 0);

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
    reader: &mut BitReader,
) -> anyhow::Result<BitVec<Msb0, u8>> {
    let bits_expected = w_tiles as usize * 8 * h_tiles as usize * 8;
    let mut writer = BitWriter::new();
    let mut packet_type: PacketType = reader.deku_read()?;
    loop {
        match packet_type {
            PacketType::RLE => decode_rle_packet(reader, &mut writer)?,
            PacketType::Data => decode_data_packet(reader, &mut writer)?,
        }
        packet_type = !packet_type;
        if writer.len() == bits_expected {
            break;
        } else if writer.len() > bits_expected {
            anyhow::bail!(
                "Too much data: expected {}, got {}",
                bits_expected,
                writer.len()
            );
        }
    }
    Ok(writer.bits)
}

fn decode_rle_packet(reader: &mut BitReader, writer: &mut BitWriter) -> anyhow::Result<()> {
    let pair_count = read_rle_count(reader)?;
    writer.extend_with_zeroes(2 * pair_count as usize);
    Ok(())
}

fn decode_data_packet(reader: &mut BitReader, writer: &mut BitWriter) -> anyhow::Result<()> {
    loop {
        let pair = reader.read(2)?;
        if pair == bitvec![Msb0, u8; 0, 0] {
            break;
        }
        writer.write(pair);
    }
    Ok(())
}

/// TODO: implicit data packet termination when sprite is full
///     See https://youtu.be/aF1Yw_wu2cM?t=1463
///     Should break current `empty` tests
fn compress_bitplane(
    w_tiles: u8,
    h_tiles: u8,
    reader: &mut BitReader,
    writer: &mut BitWriter,
) -> anyhow::Result<()> {
    // 0 if we are not in a run of 00s, >0 otherwise.
    let mut rle_count;
    let bits_expected = w_tiles as usize * 8 * h_tiles as usize * 8;
    let mut bits_read = 0usize;

    if let Ok(pair) = reader.read(2) {
        bits_read += 2;
        if pair == bitvec![Msb0, u8; 0, 0] {
            writer.deku_write(PacketType::RLE)?;
            rle_count = 1;
        } else {
            writer.deku_write(PacketType::Data)?;
            rle_count = 0;
            writer.write(pair);
        }
    } else {
        // Special case: empty bitplane.
        writer.deku_write(PacketType::Data)?;
        rle_count = 0;
    }

    loop {
        if let Ok(pair) = reader.read(2) {
            bits_read += 2;
            if pair == bitvec![Msb0, u8; 0, 0] {
                if rle_count == 0 {
                    // End data packet.
                    writer.extend_with_zeroes(2);
                }
                rle_count += 1;
            } else {
                if rle_count > 1 {
                    // End RLE packet.
                    write_rle_count(rle_count, writer);
                    rle_count = 0;
                }
                writer.write(pair);
            }
        } else {
            if rle_count == 0 {
                writer.extend_with_zeroes(2);
            } else {
                write_rle_count(rle_count, writer);
            }
            break;
        }
    }

    assert_eq!(
        bits_expected, bits_read,
        "Didn't read the expected amount of input"
    );

    Ok(())
}

/// See https://youtu.be/aF1Yw_wu2cM?t=1227
fn delta_encode(input: &BitSlice<Msb0, u8>) -> BitVec<Msb0, u8> {
    let mut output = bitvec![Msb0, u8;];
    let mut prev: bool = false;
    for bit in input {
        if *bit == prev {
            output.push(false);
        } else {
            output.push(true);
            prev = *bit;
        }
    }
    assert_eq!(input.len(), output.len());
    output
}

/// See https://youtu.be/aF1Yw_wu2cM?t=1250
fn delta_decode(input: &BitSlice<Msb0, u8>) -> BitVec<Msb0, u8> {
    let mut output = bitvec![Msb0, u8;];
    let mut prev: bool = false;
    for bit in input {
        if *bit {
            prev = !prev;
        }
        output.push(prev);
    }
    assert_eq!(input.len(), output.len());
    output
}

#[cfg(test)]
mod tests {
    use crate::pokepak::{
        compress_bitplane, decompress_bitplane, delta_decode, delta_encode, read_rle_count,
        write_rle_count, BitReader, BitWriter, EncodingMethod, PacketType,
    };
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

    /// Test pattern: 0x0 tiles.
    /// It'd be weird to actually use this corner case,
    /// but now it doesn't break the decoder.
    #[test]
    fn decompress_empty() {
        let expected = bitvec![Msb0, u8; 0; 0];
        let compressed = bitvec![Msb0, u8; 1, 0, 0];
        let mut reader = BitReader::new(&compressed);
        let actual = decompress_bitplane(0, 0, &mut reader).unwrap();
        assert_eq!(
            expected.len(),
            actual.len(),
            "Didn't write as much data as in original"
        );
        assert_eq!(actual, expected);
        assert_eq!(compressed.len(), reader.pos, "Didn't read entire input");
    }

    /// Test pattern: 1x1 tiles, all 0s.
    #[test]
    fn decompress_solid_0() {
        let expected = bitvec![Msb0, u8; 0; 64];
        let compressed = bitvec![Msb0, u8; 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1];
        let mut reader = BitReader::new(&compressed);
        let actual = decompress_bitplane(1, 1, &mut reader).unwrap();
        assert_eq!(
            expected.len(),
            actual.len(),
            "Didn't write as much data as in original"
        );
        assert_eq!(actual, expected);
        assert_eq!(compressed.len(), reader.pos, "Didn't read entire input");
    }

    /// Test pattern: 1x1 tiles, all 1s.
    #[test]
    fn decompress_solid_1() {
        let expected = bitvec![Msb0, u8; 1; 64];
        let compressed = bitvec![Msb0, u8;
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 0, 0,
        ];
        let mut reader = BitReader::new(&compressed);
        let actual = decompress_bitplane(1, 1, &mut reader).unwrap();
        assert_eq!(
            expected.len(),
            actual.len(),
            "Didn't write as much data as in original"
        );
        assert_eq!(actual, expected);
        assert_eq!(compressed.len(), reader.pos, "Didn't read entire input");
    }

    /// Test pattern: 1x1-tile checkerboard where upper left and bottom right quadrants are 0s and other quadrants are 1s.
    #[test]
    fn decompress_checkerboard() {
        let expected = bitvec![Msb0, u8;
            0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1,
            1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1,
            1, 1, 0, 0, 0, 0,
        ];
        assert_eq!(64, expected.len());
        let compressed = bitvec![Msb0, u8;
            0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1,
            1, 0, 0, 0, 1,
        ];
        let mut reader = BitReader::new(&compressed);
        let actual = decompress_bitplane(1, 1, &mut reader).unwrap();
        assert_eq!(
            expected.len(),
            actual.len(),
            "Didn't write as much data as in original"
        );
        assert_eq!(actual, expected);
        assert_eq!(compressed.len(), reader.pos, "Didn't read entire input");
    }

    /// See https://youtu.be/aF1Yw_wu2cM?t=991
    /// and https://youtu.be/aF1Yw_wu2cM?t=1104
    /// Example padded to 64 output bits by adding one more RLE packet.
    #[test]
    fn decompress_example() {
        let expected = bitvec![Msb0, u8;
            0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0,
            1, 0, 1, 0, 0, 0,
        ];
        assert_eq!(64, expected.len());
        let compressed = bitvec![Msb0, u8;
            0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0,
            0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0,
        ];
        let mut reader = BitReader::new(&compressed);
        let actual = decompress_bitplane(1, 1, &mut reader).unwrap();
        assert_eq!(
            expected.len(),
            actual.len(),
            "Didn't write as much data as in original"
        );
        assert_eq!(actual, expected);
        assert_eq!(compressed.len(), reader.pos, "Didn't read entire input");
    }

    /// Test pattern: 0x0 tiles.
    /// It'd be weird to actually use this corner case,
    /// but it shouldn't break the encoder either.
    #[test]
    fn compress_empty() {
        let expected = bitvec![Msb0, u8; 1, 0, 0];
        let data = bitvec![Msb0, u8; 0; 0];
        let mut reader = BitReader::new(&data);
        let mut writer = BitWriter::new();
        compress_bitplane(0, 0, &mut reader, &mut writer).unwrap();
        let actual = writer.bits;
        assert_eq!(
            expected.len(),
            actual.len(),
            "Didn't write as much data as in original"
        );
        assert_eq!(actual, expected);
        assert_eq!(data.len(), reader.pos, "Didn't read entire input");
    }

    /// Test pattern: 1x1 tiles, all 0s.
    #[test]
    fn compress_solid_0() {
        let expected = bitvec![Msb0, u8; 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1];
        let data = bitvec![Msb0, u8; 0; 64];
        let mut reader = BitReader::new(&data);
        let mut writer = BitWriter::new();
        compress_bitplane(1, 1, &mut reader, &mut writer).unwrap();
        let actual = writer.bits;
        assert_eq!(
            expected.len(),
            actual.len(),
            "Didn't write as much data as in original"
        );
        assert_eq!(actual, expected);
        assert_eq!(data.len(), reader.pos, "Didn't read entire input");
    }

    /// Test pattern: 1x1 tiles, all 0s.
    #[test]
    fn compress_solid_1() {
        let expected = bitvec![Msb0, u8;
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 0, 0,
        ];
        let data = bitvec![Msb0, u8; 1; 64];
        let mut reader = BitReader::new(&data);
        let mut writer = BitWriter::new();
        compress_bitplane(1, 1, &mut reader, &mut writer).unwrap();
        let actual = writer.bits;
        assert_eq!(
            expected.len(),
            actual.len(),
            "Didn't write as much data as in original"
        );
        assert_eq!(actual, expected);
        assert_eq!(data.len(), reader.pos, "Didn't read entire input");
    }

    /// Test pattern: 1x1-tile checkerboard where upper left and bottom right quadrants are 0s and other quadrants are 1s.
    #[test]
    fn compress_checkerboard() {
        let expected = bitvec![Msb0, u8;
            0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1,
            1, 0, 0, 0, 1,
        ];
        let data = bitvec![Msb0, u8;
            0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1,
            1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1,
            1, 1, 0, 0, 0, 0,
        ];
        let mut reader = BitReader::new(&data);
        let mut writer = BitWriter::new();
        compress_bitplane(1, 1, &mut reader, &mut writer).unwrap();
        let actual = writer.bits;
        assert_eq!(
            expected.len(),
            actual.len(),
            "Didn't write as much data as in original"
        );
        assert_eq!(actual, expected);
        assert_eq!(data.len(), reader.pos, "Didn't read entire input");
    }

    /// See https://youtu.be/aF1Yw_wu2cM?t=991
    /// and https://youtu.be/aF1Yw_wu2cM?t=1104
    /// Example padded to 64 output bits by adding one more RLE packet.
    #[test]
    fn compress_example() {
        let expected = bitvec![Msb0, u8;
            0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0,
            0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0,
        ];
        let data = bitvec![Msb0, u8;
            0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0,
            1, 0, 1, 0, 0, 0,
        ];
        let mut reader = BitReader::new(&data);
        let mut writer = BitWriter::new();
        compress_bitplane(1, 1, &mut reader, &mut writer).unwrap();
        let actual = writer.bits;
        assert_eq!(
            expected.len(),
            actual.len(),
            "Didn't write as much data as in original"
        );
        assert_eq!(actual, expected);
        assert_eq!(data.len(), reader.pos, "Didn't read entire input");
    }

    /// See https://youtu.be/aF1Yw_wu2cM?t=1272
    #[test]
    fn delta_encode_example() {
        let expected = bitvec![Msb0, u8;
            0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0];
        let data = bitvec![Msb0, u8;
            0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0];
        let actual = delta_encode(&data);
        assert_eq!(actual, expected);
    }

    /// See https://youtu.be/aF1Yw_wu2cM?t=1272
    #[test]
    fn delta_decode_example() {
        let expected = bitvec![Msb0, u8;
            0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0];
        let delta_data = bitvec![Msb0, u8;
            0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0];
        let actual = delta_decode(&delta_data);
        assert_eq!(actual, expected);
    }
}

/// 2-bit image packed to 4 bits per pixel.
struct Image2Bit {
    width: usize,
    height: usize,
    bits: BitVec<Msb0, u8>,
}

// TODO: we should be able to treat any image type as an input so long as it has the right number of colors.
impl Image2Bit {
    fn read(path: &Path) -> anyhow::Result<Self> {
        let decoder = Decoder::new(File::open(path)?);
        let mut reader = decoder.read_info()?;

        let info = reader.info();
        let width = info.width as usize;
        let height = info.height as usize;
        let bit_depth = info.bit_depth as usize;

        // Check palette preconditions
        match info.color_type {
            ColorType::Grayscale => match info.bit_depth {
                BitDepth::One | BitDepth::Two => (),
                bit_depth => {
                    anyhow::bail!("Illegal bit depth for 4 greys: {:?}", bit_depth)
                }
            },
            ColorType::Indexed => {
                if let Some(palette) = info.palette.as_ref() {
                    if palette.len() % 3 != 0 {
                        anyhow::bail!("Malformed palette");
                    }
                    let num_colors = palette.len() / 3;
                    if num_colors != 4 {
                        anyhow::bail!("Expected exactly 4 colors, found {}", num_colors);
                    }
                }
                match info.bit_depth {
                    BitDepth::One | BitDepth::Two | BitDepth::Four | BitDepth::Eight => (),
                    bit_depth => {
                        anyhow::bail!("Illegal palette bit depth for 4 colors: {:?}", bit_depth)
                    }
                }
            }
            color_type => {
                anyhow::bail!("Unsupported bit depth for 4 of anything: {:?}", color_type)
            }
        }

        // Read input image into a single bitvec, scanline by scanline
        let png_pixels_per_byte = 8 / bit_depth;
        let mut bits = bitvec![Msb0, u8;];
        while let Some(row) = reader.next_row()? {
            for (i, byte) in row.data().iter().enumerate() {
                let byte = byte.view_bits::<Msb0>();
                for p in 0..png_pixels_per_byte {
                    let x = i * png_pixels_per_byte + p;
                    if x >= width {
                        // Past end of last partial byte of the scanline
                        break;
                    }
                    if bit_depth == 1 {
                        // MSB is always zero
                        bits.push(false);
                    }
                    bits.extend(&byte[(p * bit_depth)..((p + 1) * bit_depth)]);
                }
            }
        }

        assert_eq!(width * height * 2, bits.len());

        Ok(Image2Bit {
            width,
            height,
            bits,
        })
    }

    fn write(&self, path: &Path) -> anyhow::Result<()> {
        let mut bytes = vec![];

        let mut x = 0usize;
        let mut packed = 0u8;
        for c in self.bits.chunks(2) {
            packed <<= 2;
            packed |= c.load_be::<u8>();
            if x % 4 == 4 - 1 {
                // Byte full
                bytes.push(packed);
            } else if x % self.width == self.width - 1 {
                // Pad partial byte at end of scanline
                packed <<= 2 * (4 - (self.width % 4));
                bytes.push(packed);
            }
            x += 1;
        }

        let mut encoder = Encoder::new(
            BufWriter::new(File::create(path)?),
            self.width as u32,
            self.height as u32,
        );
        encoder.set_color(ColorType::Indexed);
        encoder.set_depth(BitDepth::Two);
        // Default WASM-4 palette
        encoder.set_palette(vec![
            0xe0, 0xf8, 0xcf, 0x86, 0xc0, 0x6c, 0x30, 0x68, 0x50, 0x07, 0x18, 0x21,
        ]);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&bytes)?;

        Ok(())
    }
}

pub fn encode(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    let img = Image2Bit::read(input_path)?;

    if img.width % 8 != 0 || img.height % 8 != 0 {
        anyhow::bail!("Image size must be a multiple of 8 in each direction (for now)")
    }
    let w_tiles = (img.width / 8) as u8;
    let h_tiles = (img.height / 8) as u8;

    // Most significant bitplane
    let mut most_significant_bitplane = bitvec![Msb0, u8;];
    // Least significant bitplane
    let mut least_significant_bitplane = bitvec![Msb0, u8;];
    for c in img.bits.chunks(2) {
        most_significant_bitplane.push(c[0]);
        least_significant_bitplane.push(c[1]);
    }

    let mut best_len: Option<usize> = None;
    let mut best_bytes: Option<Vec<u8>> = None;

    // Try all 6 encoding options.
    for primary_buffer in [
        PrimaryBuffer::MostSignificantBitplane,
        // PrimaryBuffer::LeastSignificantBitplane,
    ] {
        let (bp0, bp1) = match primary_buffer {
            PrimaryBuffer::MostSignificantBitplane => {
                (&most_significant_bitplane, &least_significant_bitplane)
            }
            PrimaryBuffer::LeastSignificantBitplane => {
                (&least_significant_bitplane, &most_significant_bitplane)
            }
        };

        let sprite_header = SpriteHeader {
            w_tiles,
            h_tiles,
            primary_buffer,
        };

        for encoding_method in [
            EncodingMethod::MODE_1,
            // EncodingMethod::MODE_2,
            // EncodingMethod::MODE_3,
        ] {
            let second_bitplane_header = SecondBitplaneHeader { encoding_method };

            let encoded_bp0 = delta_encode(bp0);

            let xored_bp1 = if encoding_method.xor_bp1_with_bp0() {
                // All the bitvec bit-ops traits require a mutable reference to the left-hand side, which is not how non-assignment bit ops are supposed to work!
                let xored_bp1 = bp1.clone() ^ bp0.iter().by_val();
                assert_eq!(bp1.len(), xored_bp1.len());
                assert_eq!(bp0.len(), xored_bp1.len());
                Some(xored_bp1)
            } else {
                None
            };
            let bp1_stage1 = xored_bp1.as_ref().unwrap_or(&bp1);

            let encoded_bp1 = if encoding_method.delta_encode_bp1() {
                Some(delta_encode(bp1_stage1))
            } else {
                None
            };
            let bp1_stage2 = encoded_bp1.as_ref().unwrap_or(bp1_stage1);

            let mut writer = BitWriter::new();

            writer.deku_write(&sprite_header)?;
            compress_bitplane(
                w_tiles,
                h_tiles,
                &mut BitReader::new(&encoded_bp0),
                &mut writer,
            )?;

            writer.deku_write(&second_bitplane_header)?;
            compress_bitplane(
                w_tiles,
                h_tiles,
                &mut BitReader::new(bp1_stage2),
                &mut writer,
            )?;

            let bytes = writer.bits.as_raw_slice();
            let len = bytes.len();
            if best_len.is_none() || len < best_len.unwrap() {
                best_len = Some(len);
                best_bytes = Some(Vec::from(bytes));
            }
        }
    }

    fs::write(output_path, best_bytes.unwrap())?;

    Ok(())
}

pub fn decode(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    let bytes = fs::read(input_path)?;
    let mut reader = BitReader::new(bytes.view_bits());

    let sprite_header: SpriteHeader = reader.deku_read()?;
    let encoded_bp0 =
        decompress_bitplane(sprite_header.w_tiles, sprite_header.h_tiles, &mut reader)?;
    println!("encoded_bp0.len = {}", encoded_bp0.len());

    let bp0 = delta_decode(&encoded_bp0);
    println!("bp0.len = {}", bp0.len());

    let second_bitplane_header: SecondBitplaneHeader = reader.deku_read()?;
    let bp1_stage2 =
        decompress_bitplane(sprite_header.w_tiles, sprite_header.h_tiles, &mut reader)?;
    println!("bp1_stage2.len = {}", bp1_stage2.len());

    let decoded_bp1 = if second_bitplane_header.encoding_method.delta_encode_bp1() {
        Some(delta_decode(&bp1_stage2))
    } else {
        None
    };
    let bp1_stage1 = decoded_bp1.as_ref().unwrap_or(&bp1_stage2);
    println!("bp1_stage1.len = {}", bp1_stage1.len());

    let xored_bp1 = if second_bitplane_header.encoding_method.xor_bp1_with_bp0() {
        Some(bp1_stage1.clone() ^ bp0.iter().by_val())
    } else {
        None
    };
    let bp1 = xored_bp1.as_ref().unwrap_or(bp1_stage1);
    println!("bp1.len = {}", bp1.len());

    if bp0.len() != bp1.len() {
        anyhow::bail!(
            "Bitplane length mismatch: bp0 = {} bits, bp1 = {} bits, delta_encode_bp1 = {}, xor_bp1_with_bp0 = {}",
            bp0.len(),
            bp1.len(),
            second_bitplane_header.encoding_method.delta_encode_bp1(),
            second_bitplane_header.encoding_method.xor_bp1_with_bp0()
        )
    }

    let (most_significant_bitplane, least_significant_bitplane) = match sprite_header.primary_buffer
    {
        PrimaryBuffer::MostSignificantBitplane => (&bp0, bp1),
        PrimaryBuffer::LeastSignificantBitplane => (bp1, &bp0),
    };

    let mut bits = bitvec![Msb0, u8;];
    for (m, l) in most_significant_bitplane
        .iter()
        .by_val()
        .zip(least_significant_bitplane.iter().by_val())
    {
        bits.push(m);
        bits.push(l);
    }

    let img = Image2Bit {
        width: sprite_header.w_tiles as usize * 8,
        height: sprite_header.h_tiles as usize * 8,
        bits,
    };
    img.write(output_path)?;

    Ok(())
}
