mod lo5;

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
    Lo5 {
        /// Input image.
        #[clap(value_parser)]
        input: PathBuf,
        /// Output directory. Output filenames will be generated from the input filename.
        #[clap(value_parser)]
        output: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Lo5 { input, output } => lo5::convert(input.as_path(), output.as_path())?,
    }
    Ok(())
}
