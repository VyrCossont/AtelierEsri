use anyhow::{anyhow, bail, Result};
use image::{GenericImageView, ImageReader};
use std::path::Path;

pub struct CustomCharacter {
    pub group_name: Option<String>,
    pub name: Option<String>,
    pub bytes: [u8; 8],
}

impl CustomCharacter {
    pub fn load(group_name: &str, png_path: &Path, threshold: u8) -> Result<Self> {
        let image = ImageReader::open(png_path)?.decode()?;
        let name = png_path
            .file_stem()
            .ok_or(anyhow!("Couldn't get file stem for PNG path"))?
            .to_string_lossy();
        Ok(Self {
            group_name: Some(group_name.to_string()),
            name: Some(name.to_string()),
            bytes: Self::bytes_from_image(image, threshold)?,
        })
    }

    pub fn from_image(
        image: impl GenericImageView<Pixel = image::Rgba<u8>>,
        threshold: u8,
    ) -> Result<Self> {
        Ok(Self {
            group_name: None,
            name: None,
            bytes: Self::bytes_from_image(image, threshold)?,
        })
    }

    /// Generate a 4x4 block centered in the 8x8 area.
    #[rustfmt::skip]
    pub fn rectangle() -> Self {
        Self {
            group_name: None,
            name: None,
            bytes: [
                0b00000000,
                0b00000000,
                0b00111100,
                0b00111100,
                0b00111100,
                0b00111100,
                0b00000000,
                0b00000000,
            ],
        }
    }

    /// `threshold` is for the alpha channel:
    /// - if 255, fully opaque pixels are set, others are cleared.
    /// - if 1, fully transparent pixels are cleared, others are set.
    fn bytes_from_image(
        image: impl GenericImageView<Pixel = image::Rgba<u8>>,
        threshold: u8,
    ) -> Result<[u8; 8]> {
        if image.width() > 8 || image.height() > 8 {
            bail!(
                "wrong image size for PICO-8 custom character: {}x{}",
                image.width(),
                image.height(),
            );
        }

        let mut bytes = [0u8; 8];
        for y in 0..image.height() {
            for x in 0..image.width() {
                let image::Rgba([_, _, _, a]) = image.get_pixel(x, y);
                if a >= threshold {
                    bytes[y as usize] |= 1 << x;
                }
            }
        }
        Ok(bytes)
    }

    // todo: there must be byte string formatting *somewhere*

    /// Number of Lua characters in a compact custom character string.
    pub const COMPACT_LUA_LEN: usize = 10;

    /// Returns a compact custom character string.
    /// Likely to not be valid Lua.
    /// Might be most compact by byte count but not as rendered by P8's code editor.
    /// See: PICO-8 manual, "Appendix A: P8SCII Control Codes"
    /// See: <https://pico-8.fandom.com/wiki/P8SCII>
    fn p8scii_compact(&self) -> String {
        let prefix = br"\^.";
        // TODO: this has a bug with decimal escapes: for example, writing [0x02, 0x38, 0x38]
        //  results in the Lua string "\288" which PICO-8 interprets as a too-large decimal escape.
        //  Hex escapes might be better. Finding the *most* compact encoding is left as an exercise.
        let mut bytes = Vec::<u8>::with_capacity(prefix.len() + 4 * self.bytes.len());
        bytes.extend_from_slice(prefix);
        for b in self.bytes {
            match b {
                // Lua or PICO-8 specific escapes.
                0x00..=0x0d => bytes.extend_from_slice(match b {
                    0x00 => br"\0",
                    0x01 => br"\*",
                    0x02 => br"\#",
                    0x03 => br"\-",
                    0x04 => br"\|",
                    0x05 => br"\+",
                    0x06 => br"\^",
                    0x07 => br"\a",
                    0x08 => br"\b",
                    0x09 => br"\t",
                    0x0a => br"\n",
                    0x0b => br"\v",
                    0x0c => br"\f",
                    0x0d => br"\r",
                    _ => panic!("impossible"),
                }),
                // Printable characters that overlap with normal ASCII.
                b' '..=b'~' => bytes.push(b),
                // Decimal escapes.
                _ => bytes.extend_from_slice(format!(r"\{b}").into_bytes().as_slice()),
            }
        }
        String::from_utf8_lossy(bytes.as_slice()).to_string()
    }

    /// Number of Lua characters in a hex custom character string.
    pub const HEX_LUA_LEN: usize = 18;

    /// Returns a hex custom character string.
    fn p8scii_hex(&self) -> String {
        format!(
            "\\^:{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.bytes[0],
            self.bytes[1],
            self.bytes[2],
            self.bytes[3],
            self.bytes[4],
            self.bytes[5],
            self.bytes[6],
            self.bytes[7],
        )
    }

    pub fn lua_line(&self, compact: bool) -> Option<String> {
        let Some(name) = &self.name else {
            return None;
        };

        let p8scii = if compact {
            self.p8scii_compact()
        } else {
            self.p8scii_hex()
        };

        Some(if let Some(group_name) = &self.group_name {
            format!("{group_name}_{name} = \"{p8scii}\"\n")
        } else {
            format!("{name} = \"{p8scii}\"\n")
        })
    }
}

impl Default for CustomCharacter {
    fn default() -> Self {
        Self {
            group_name: None,
            name: None,
            bytes: [0; 8],
        }
    }
}
