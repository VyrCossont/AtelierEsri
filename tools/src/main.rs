mod grey_quantizer;
mod image2bit;
mod implicit_tree;
mod items;
mod lo5;
mod pokepak;
mod tileshred;

use anyhow;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Tool for working with resources for WASM-4 ROMs.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Split a 4-color + 1-transparent-color PNG into a 4-color and a 2-color PNG.
    Lo5 {
        /// Input image.
        #[clap(value_parser)]
        input: PathBuf,
        /// Output directory. Output filenames will be generated from the input filename.
        #[clap(value_parser)]
        output: PathBuf,
    },
    /// Encode a 4-color PNG to Poképak.
    PokepakEncode {
        /// Input image.
        #[clap(value_parser)]
        input: PathBuf,
        /// Output image.
        #[clap(value_parser)]
        output: PathBuf,
    },
    /// Decode a Poképak image to a 4-color PNG.
    PokepakDecode {
        /// Input image.
        #[clap(value_parser)]
        input: PathBuf,
        /// Output image.
        #[clap(value_parser)]
        output: PathBuf,
    },
    /// Color-quantize a sprite sheet, tile by tile.
    /// Intended for items, not tile maps.
    Tileshred {
        /// Tile width.
        #[clap(value_parser)]
        tile_width: u32,
        /// Tile height.
        #[clap(value_parser)]
        tile_height: u32,
        /// Input image.
        #[clap(value_parser)]
        input: PathBuf,
        /// Output image.
        #[clap(value_parser)]
        output: PathBuf,
    },
    /// Generate a JSON schema for items structures.
    ItemsSchema {
        /// Output JSON file.
        #[clap(value_parser)]
        output: PathBuf,
    },
    /// Generate Rust code from an items JSON file.
    ItemsCode {
        /// Input JSON file.
        #[clap(value_parser)]
        input: PathBuf,
        /// Output Rust file.
        #[clap(value_parser)]
        output: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Lo5 { input, output } => lo5::convert(input.as_path(), output.as_path())?,
        Commands::PokepakEncode { input, output } => {
            pokepak::encode(input.as_path(), output.as_path())?
        }
        Commands::PokepakDecode { input, output } => {
            pokepak::decode(input.as_path(), output.as_path())?
        }
        Commands::Tileshred {
            tile_width,
            tile_height,
            input,
            output,
        } => tileshred::convert(tile_width, tile_height, input.as_path(), output.as_path())?,
        Commands::ItemsSchema { output } => items::schema(output.as_path())?,
        Commands::ItemsCode { input, output } => items::code(input.as_path(), output.as_path())?,
    }
    Ok(())
}
