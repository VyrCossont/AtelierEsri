//! PICO-8 asset format support.

use crate::fsutil::{delete_dir, ensure_dir};
use std::path::Path;

pub fn generate_assets(asset_base_dir: &Path, build_dir: &Path) -> anyhow::Result<()> {
    delete_dir(build_dir)?;
    ensure_dir(build_dir)?;

    // TODO: everything

    Ok(())
}
