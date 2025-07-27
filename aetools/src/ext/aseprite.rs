use serde::Deserialize;
use std::fs::File;
use std::path::Path;
use std::process::Command;

const ASEPRITE: &str = "aseprite";

/// Export an Aseprite file to a single image.
pub fn export(input: &Path, output: &Path) -> anyhow::Result<()> {
    let status = Command::new(ASEPRITE)
        .arg("--batch")
        .arg(input)
        .arg("--save-as")
        .arg(output)
        .status()?;
    if !status.success() {
        anyhow::bail!("{ASEPRITE} exited with code {status}");
    }
    Ok(())
}

/// Export an Aseprite file to a PNG for each slice.
pub fn export_slices(input: &Path, output_dir: &Path) -> anyhow::Result<()> {
    let status = Command::new(ASEPRITE)
        .arg("--batch")
        .arg(input)
        .arg("--save-as")
        .arg(output_dir.join("{slice}.png"))
        .status()?;
    if !status.success() {
        anyhow::bail!("{ASEPRITE} exited with code {status}");
    }
    Ok(())
}

/// Export sprite metadata from an Aseprite file.
pub fn export_metadata(input: &Path, output: &Path) -> anyhow::Result<()> {
    let status = Command::new(ASEPRITE)
        .arg("--batch")
        .arg("--list-slices")
        .arg(input)
        .arg("--data")
        .arg(output)
        .status()?;
    if !status.success() {
        anyhow::bail!("{ASEPRITE} exited with code {status}");
    }
    Ok(())
}

/// Read data written by [export_metadata].
pub fn read_metadata(input: &Path) -> anyhow::Result<Project> {
    let project = serde_json::from_reader(File::open(input)?)?;
    Ok(project)
}

/// Top-level sprite info JSON for an Aseprite project.
///
/// https://www.aseprite.org/docs/cli#data
#[derive(Debug, Deserialize)]
pub struct Project {
    pub meta: Meta,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub slices: Vec<Slice>,
}

#[derive(Debug, Deserialize)]
pub struct Slice {
    pub name: String,
    pub keys: Vec<SliceKey>,
}

#[derive(Debug, Deserialize)]
pub struct SliceKey {
    /// 9-patch data. Origin relative to bounds.
    pub center: Option<Rect>,
}

#[derive(Debug, Deserialize)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}
