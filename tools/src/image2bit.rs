use deku::bitvec::*;
use png::{BitDepth, ColorType, Decoder, Encoder};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// 2-bit image packed to 4 bits per pixel.
pub struct Image2Bit {
    width: usize,
    height: usize,
    bits: BitVec<Msb0, u8>,
}

// TODO: we should be able to treat any image type as an input so long as it has the right number of colors.
impl Image2Bit {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            bits: bitvec![Msb0, u8; 0; 2 * width * height],
        }
    }

    pub fn from_bits(width: usize, height: usize, bits: BitVec<Msb0, u8>) -> Self {
        assert_eq!(2 * width * height, bits.len());
        Self {
            width,
            height,
            bits,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn bits(&self) -> &BitSlice<Msb0, u8> {
        &self.bits
    }

    pub fn read(path: &Path) -> anyhow::Result<Self> {
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

    pub fn write(&self, path: &Path) -> anyhow::Result<()> {
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
