//! Include the file that gets generated by the build script invoking `w4 png2src`, etc.
//! It should be included in exactly this one file, and then used from here.

include!(concat!(env!("OUT_DIR"), "/assets.rs"));

// TODO: refactor these so we generate a whole module tree in `OUT_DIR`.
pub mod cursor;
pub mod element;
pub mod item;
