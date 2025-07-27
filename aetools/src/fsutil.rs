use std::fs;
use std::io::ErrorKind;
use std::path::Path;

pub fn delete_dir(path: &Path) -> anyhow::Result<()> {
    if let Err(err) = fs::remove_dir_all(path) {
        if err.kind() != ErrorKind::NotFound {
            anyhow::bail!(
                "Couldn't remove directory {}: {}",
                path.to_string_lossy(),
                err
            );
        }
    }
    Ok(())
}

pub fn ensure_dir(path: &Path) -> anyhow::Result<()> {
    if let Err(err) = fs::create_dir_all(path) {
        if err.kind() != ErrorKind::AlreadyExists {
            anyhow::bail!(
                "Couldn't create directory {}: {}",
                path.to_string_lossy(),
                err
            );
        }
    }
    Ok(())
}
