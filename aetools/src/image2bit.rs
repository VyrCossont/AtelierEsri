use deku::bitvec::*;
use png::{BitDepth, ColorType, Decoder, Encoder};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// 2-bit image packed to 4 pixels per byte in memory.
/// Stored this way so it can be passed to compressors like Poképak.
pub struct Image2Bit {
    width: u32,
    height: u32,
    bits: BitVec<Msb0, u8>,
}

pub enum WriteMode2Bit {
    Grayscale,
    WASM4Palette,
}

pub trait PixelAccess2Bit {
    fn width(&self) -> u32;

    fn height(&self) -> u32;

    fn get_pixel(&self, x: u32, y: u32) -> u8;

    fn set_pixel(&mut self, x: u32, y: u32, pixel: u8);
}

// TODO: we should be able to treat any image type as an input so long as it has the right number of colors.
impl Image2Bit {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            bits: bitvec![Msb0, u8; 0; (2 * width * height) as usize],
        }
    }

    pub fn from_bits(width: u32, height: u32, bits: BitVec<Msb0, u8>) -> Self {
        assert_eq!(2 * width * height, bits.len() as u32);
        Self {
            width,
            height,
            bits,
        }
    }

    pub fn bits(&self) -> &BitSlice<Msb0, u8> {
        &self.bits
    }

    fn pixel_offset(&self, x: u32, y: u32) -> usize {
        assert!(
            x < self.width,
            "x out of bounds: {x} ≥ {width}",
            width = self.width
        );
        assert!(
            y < self.height,
            "y out of bounds: {y} ≥ {height}",
            height = self.height
        );
        (2 * (y * self.width + x)) as usize
    }

    pub fn subimage<'a>(&'a mut self, x: u32, y: u32, width: u32, height: u32) -> Subimage2Bit<'a> {
        assert!(
            x + width <= self.width,
            "x + width out of bounds: {x_plus_width} > {width}",
            x_plus_width = x + width,
            width = self.width
        );
        assert!(
            y + height <= self.height,
            "y + height out of bounds: {y_plus_height} > {height}",
            y_plus_height = y + height,
            height = self.height
        );
        Subimage2Bit::<'a> {
            x,
            y,
            width,
            height,
            parent: self,
        }
    }

    pub fn read(path: &Path) -> anyhow::Result<Self> {
        let decoder = Decoder::new(File::open(path)?);
        let mut reader = decoder.read_info()?;

        let info = reader.info();
        let width = info.width;
        let height = info.height;
        let bit_depth = info.bit_depth as u8;

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
                    let x = (i as u32 * png_pixels_per_byte as u32) + p as u32;
                    if x >= width {
                        // Past end of last partial byte of the scanline
                        break;
                    }
                    if bit_depth == 1 {
                        // MSB is always zero
                        bits.push(false);
                    }
                    bits.extend(&byte[(p * bit_depth) as usize..((p + 1) * bit_depth) as usize]);
                }
            }
        }

        assert_eq!(width * height * 2, bits.len() as u32);

        Ok(Image2Bit {
            width,
            height,
            bits,
        })
    }

    pub fn write(&self, path: &Path, write_mode: WriteMode2Bit) -> anyhow::Result<()> {
        let mut bytes = vec![];

        let mut x = 0u32;
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

        let mut encoder =
            Encoder::new(BufWriter::new(File::create(path)?), self.width, self.height);
        encoder.set_depth(BitDepth::Two);
        match write_mode {
            WriteMode2Bit::Grayscale => {
                encoder.set_color(ColorType::Grayscale);
            }
            WriteMode2Bit::WASM4Palette => {
                encoder.set_color(ColorType::Indexed);
                // Default WASM-4 palette
                encoder.set_palette(vec![
                    0x07, 0x18, 0x21, 0x30, 0x68, 0x50, 0x86, 0xc0, 0x6c, 0xe0, 0xf8, 0xcf,
                ]);
            }
        }

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&bytes)?;

        Ok(())
    }
}

impl PixelAccess2Bit for Image2Bit {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn get_pixel(&self, x: u32, y: u32) -> u8 {
        let offset = self.pixel_offset(x, y);
        self.bits[offset..offset + 2].load_be()
    }

    fn set_pixel(&mut self, x: u32, y: u32, pixel: u8) {
        assert!(pixel < 4, "pixel value out of bounds: {pixel} ≥ 4");
        let offset = self.pixel_offset(x, y);
        self.bits[offset..offset + 2].store_be(pixel)
    }
}

pub struct Subimage2Bit<'a> {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    parent: &'a mut Image2Bit,
}

impl PixelAccess2Bit for Subimage2Bit<'_> {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn get_pixel(&self, x: u32, y: u32) -> u8 {
        self.parent.get_pixel(x + self.x, y + self.y)
    }

    fn set_pixel(&mut self, x: u32, y: u32, pixel: u8) {
        self.parent.set_pixel(x + self.x, y + self.y, pixel)
    }
}
