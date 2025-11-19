use clap::{Parser, Subcommand, ValueHint};
use std::{error::Error, path::PathBuf};

use crate::input_handler::ensure_valid_extension::{
    ensure_himg_extension, ensure_output_extension, ensure_valid_extension,
};
use crate::logic::image_compressor::ImageCompressor;

#[derive(Parser)]
#[command(author, version, about = "Compress images using Huffman coding", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Compress(CompressArgs),
    Decompress(DecompressArgs),
}

#[derive(Parser)]
pub struct CompressArgs {
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    input: PathBuf,

    #[arg(short, long, value_hint = ValueHint::FilePath)]
    output: Option<PathBuf>,
}

#[derive(Parser)]
pub struct DecompressArgs {
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    input: PathBuf,

    #[arg(short, long, value_hint = ValueHint::FilePath)]
    output: Option<PathBuf>,
}

pub fn handle_compress(args: CompressArgs) -> Result<(), Box<dyn Error>> {
    ensure_valid_extension(&args.input)?;

    let output = args
        .output
        .unwrap_or_else(|| args.input.with_extension("himg"));

    let input_display = args.input.to_string_lossy().to_string();
    let output_display = output.to_string_lossy().to_string();

    let mut compressor = ImageCompressor::new();
    compressor.compress(&input_display, &output_display)?;
    Ok(())
}

pub fn handle_decompress(args: DecompressArgs) -> Result<(), Box<dyn Error>> {
    ensure_himg_extension(&args.input)?;

    let output = args
        .output
        .unwrap_or_else(|| args.input.with_extension("png"));

    ensure_output_extension(&output)?;

    let input_display = args.input.to_string_lossy().to_string();
    let output_display = output.to_string_lossy().to_string();

    let mut compressor = ImageCompressor::new();
    compressor.decompress(&input_display, &output_display)?;
    Ok(())
}
