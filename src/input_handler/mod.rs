use clap::{Parser, Subcommand, ValueHint};
use std::{error::Error, ffi::OsStr, path::PathBuf};

use crate::logic::image_compressor::ImageCompressor;

#[derive(Parser)]
#[command(author, version, about = "Compress JPEGs using Huffman coding", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Compress(CompressArgs),
    Decompress(DecompressArgs),
}

#[derive(Parser)]
struct CompressArgs {
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    input: PathBuf,

    #[arg(short, long, value_hint = ValueHint::FilePath)]
    output: Option<PathBuf>,
}

#[derive(Parser)]
struct DecompressArgs {
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    input: PathBuf,

    #[arg(short, long, value_hint = ValueHint::FilePath)]
    output: Option<PathBuf>,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        Command::Compress(args) => handle_compress(args),
        Command::Decompress(args) => handle_decompress(args),
    }
}

fn handle_compress(args: CompressArgs) -> Result<(), Box<dyn Error>> {
    ensure_jpeg_extension(&args.input)?;

    let output = args
        .output
        .unwrap_or_else(|| args.input.with_extension("himg"));

    let input_display = args.input.to_string_lossy().to_string();
    let output_display = output.to_string_lossy().to_string();

    let mut compressor = ImageCompressor::new();
    compressor.compress(&input_display, &output_display)?;
    Ok(())
}

fn handle_decompress(args: DecompressArgs) -> Result<(), Box<dyn Error>> {
    ensure_himg_extension(&args.input)?;

    let output = args
        .output
        .unwrap_or_else(|| args.input.with_extension("jpg"));

    ensure_jpeg_output_extension(&output)?;

    let input_display = args.input.to_string_lossy().to_string();
    let output_display = output.to_string_lossy().to_string();

    let mut compressor = ImageCompressor::new();
    compressor.decompress(&input_display, &output_display)?;
    Ok(())
}

fn ensure_jpeg_extension(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    match path.extension().and_then(OsStr::to_str) {
        Some(ext) if matches!(ext.to_ascii_lowercase().as_str(), "jpg" | "jpeg") => Ok(()),
        _ => Err("Input file must have a .jpg or .jpeg extension".into()),
    }
}
fn ensure_himg_extension(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    match path.extension().and_then(OsStr::to_str) {
        Some(ext) if ext.eq_ignore_ascii_case("himg") => Ok(()),
        _ => Err("Input file must have a .himg extension".into()),
    }
}

fn ensure_jpeg_output_extension(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    match path.extension().and_then(OsStr::to_str) {
        Some(ext) if matches!(ext.to_ascii_lowercase().as_str(), "jpg" | "jpeg") => Ok(()),
        _ => Err("Output file must have a .jpg or .jpeg extension".into()),
    }
}
