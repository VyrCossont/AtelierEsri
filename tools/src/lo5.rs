use anyhow;
use divrem::DivCeil;
use png::{BitDepth, ColorType, Decoder, Encoder};
use std::ffi::OsString;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

/// Split up a 5-color PNG into a 4-color PNG and a 2-color PNG.
/// `output` must be a directory; the output filenames will be generated from the `input` filename.
pub fn convert(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    let input_stem = input_path
        .file_stem()
        .ok_or(anyhow::anyhow!("Input must be a file"))?;

    let decoder = Decoder::new(File::open(input_path)?);
    let mut reader = decoder.read_info()?;

    let info = reader.info();
    let width = info.width;
    let height = info.height;
    let num_pixels = (width * height) as usize;

    // Check palette preconditions
    if info.color_type != ColorType::Indexed {
        anyhow::bail!("Not an indexed-color image");
    }
    let input_bit_depth = info.bit_depth;
    match input_bit_depth {
        BitDepth::One | BitDepth::Two | BitDepth::Sixteen => {
            anyhow::bail!("Illegal bit depth for 5 colors")
        }
        _ => (),
    }
    let input_palette = info
        .palette
        .as_ref()
        .ok_or(anyhow::anyhow!("Missing palette"))?;
    if input_palette.len() % 3 != 0 {
        anyhow::bail!("Malformed palette");
    }
    let input_num_colors = input_palette.len() / 3;
    if input_num_colors != 5 {
        anyhow::bail!("Expected exactly 5 colors");
    }

    // Check transparency preconditions
    let input_trns = info
        .trns
        .as_ref()
        .ok_or(anyhow::anyhow!("Missing transparency chunk"))?;
    if input_trns.len() > input_num_colors {
        anyhow::bail!("Too many transparency entries");
    }
    let mut input_transparent_color_index: Option<usize> = None;
    for (i, alpha) in input_trns.into_iter().enumerate() {
        match *alpha {
            u8::MIN => {
                if input_transparent_color_index.is_some() {
                    anyhow::bail!("Expected exactly one transparent color, found more than one");
                } else {
                    input_transparent_color_index = Some(i)
                }
            }
            u8::MAX => (),
            _ => {
                anyhow::bail!("Expected all colors to be either fully transparent or fully opaque")
            }
        }
    }
    let input_transparent_color_index = input_transparent_color_index.ok_or(anyhow::anyhow!(
        "Expected exactly one transparent color, found none"
    ))? as u8;
    let last_color_index: u8 = if input_transparent_color_index == 4 {
        3
    } else {
        4
    };

    // Set up palettes for split images
    let mut palette_lo4 = vec![0; 3 * 4];
    for channel in 0..3 {
        palette_lo4[channel] = 0;
    }
    let mut p = 1usize;
    for i in 0..5 {
        if i != input_transparent_color_index && i != last_color_index {
            for channel in 0..3 {
                palette_lo4[p * 3 + channel] = input_palette[i as usize * 3 + channel];
            }
            p += 1;
        }
    }

    let mut palette_hi2 = vec![0; 3 * 2];
    for channel in 0..3 {
        palette_hi2[channel] = 0;
    }
    for channel in 0..3 {
        palette_hi2[3 + channel] = input_palette[last_color_index as usize * 3 + channel];
    }

    // Read input image
    let mut input_buf = vec![0; reader.output_buffer_size()];
    let input_frame_info = reader.next_frame(&mut input_buf)?;
    let input_bytes = &input_buf[..input_frame_info.buffer_size()];

    // Prepare to write output images with remapped colors
    let lo4_map = |c: u8| match c {
        _ if c == input_transparent_color_index => 0u8,
        _ if c == last_color_index => 0u8,
        _ if c < input_transparent_color_index => c,
        _ => c - 1,
    };
    let mut lo4_buf =
        vec![0u8; <usize as DivCeil>::div_ceil(num_pixels * BitDepth::Two as usize, 8)];
    let mut lo4_packed = 0u8;
    let mut lo4_write = |i: usize, c: u8| {
        lo4_packed <<= 2;
        lo4_packed |= lo4_map(c);
        if i % 4 == 4 - 1 || i % width as usize == width as usize - 1 {
            lo4_buf[i / 4] = lo4_packed;
        }
    };

    let hi2_map = |c: u8| match c {
        _ if c == last_color_index => 1u8,
        _ => 0u8,
    };
    let mut hi2_buf =
        vec![0u8; <usize as DivCeil>::div_ceil(num_pixels * BitDepth::One as usize, 8)];
    let mut hi2_packed = 0u8;
    let mut hi2_write = |i: usize, c: u8| {
        hi2_packed <<= 1;
        hi2_packed |= hi2_map(c);
        if i % 8 == 8 - 1 || i % width as usize == width as usize - 1 {
            hi2_buf[i / 8] = hi2_packed;
        }
    };

    if input_bit_depth == BitDepth::Four {
        for (i, c) in input_bytes.into_iter().enumerate() {
            let i_left = 2 * i;
            let c_left = *c >> 4;
            lo4_write(i_left, c_left);
            hi2_write(i_left, c_left);
            let i_right = 1 + 2 * i;
            let c_right = *c & 0xf;
            lo4_write(i_right, c_right);
            hi2_write(i_right, c_right);
        }
    } else {
        for (i, c) in input_bytes.into_iter().enumerate() {
            lo4_write(i, *c);
            hi2_write(i, *c);
        }
    }

    let mut output_filename_lo4 = OsString::from(input_stem);
    output_filename_lo4.push("-T123.png");
    let mut output_path_lo4 = PathBuf::from(output_path);
    output_path_lo4.push(output_filename_lo4);

    let mut encoder_lo4 = Encoder::new(
        BufWriter::new(File::create(output_path_lo4)?),
        width,
        height,
    );
    encoder_lo4.set_color(ColorType::Indexed);
    encoder_lo4.set_depth(BitDepth::Two);
    encoder_lo4.set_palette(palette_lo4);
    encoder_lo4.set_trns(vec![0x00]);
    let mut writer_lo4 = encoder_lo4.write_header()?;
    writer_lo4.write_image_data(&lo4_buf)?;

    let mut output_filename_hi2 = OsString::from(input_stem);
    output_filename_hi2.push("-T4.png");
    let mut output_path_hi2 = PathBuf::from(output_path);
    output_path_hi2.push(output_filename_hi2);

    let mut encoder_hi2 = Encoder::new(
        BufWriter::new(File::create(output_path_hi2)?),
        width,
        height,
    );
    encoder_hi2.set_color(ColorType::Indexed);
    encoder_hi2.set_depth(BitDepth::One);
    encoder_hi2.set_palette(palette_hi2);
    encoder_hi2.set_trns(vec![0x00]);
    let mut writer_hi2 = encoder_hi2.write_header()?;
    writer_hi2.write_image_data(&hi2_buf)?;

    Ok(())
}
