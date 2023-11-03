mod palette;
pub mod resource;

/// Four-character code used by many Apple APIs.
/// Usually human-readable but will be in MacRoman character set.
pub type OSType = [u8; 4];
