use std::path::Path;
use std::process::Command;

const MAGICK: &str = "magick";

/// Convert an image to another format (controlled by file extensions).
pub fn convert(input: &Path, output: &Path) -> anyhow::Result<()> {
    let status = Command::new(MAGICK).arg(input).arg(output).status()?;
    if !status.success() {
        anyhow::bail!("{MAGICK} exited with code {status}");
    }
    Ok(())
}

/// Return whether an image is fully opaque.
pub fn opaque(input: &Path) -> anyhow::Result<bool> {
    let output = Command::new(MAGICK)
        .args(["identify", "-format", "%[opaque]"])
        .arg(input)
        .output()?;
    if !output.status.success() {
        anyhow::bail!("{MAGICK} exited with code {status}", status = output.status);
    }
    Ok(output.stdout.as_slice() == b"True")
}

/// Extract an image's alpha channel as a 1-bit mask image.
/// Note that masks for QuickDraw `CopyMask` are inverted: black pixels are copied, white pixels are ignored.
pub fn mask(input: &Path, output: &Path, invert: bool) -> anyhow::Result<()> {
    let mut flags = vec!["-alpha", "extract", "-monochrome"];
    if invert {
        flags.push("-negate");
    }
    let status = Command::new(MAGICK)
        .arg(input)
        .args(flags)
        .arg(output)
        .status()?;
    if !status.success() {
        anyhow::bail!("{MAGICK} exited with code {status}");
    }
    Ok(())
}
