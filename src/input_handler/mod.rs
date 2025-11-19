use crate::input_handler::cli::{Cli, Command, handle_compress, handle_decompress};
use clap::Parser;
use std::error::Error;

pub mod cli;
pub mod ensure_valid_extension;

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        Command::Compress(args) => handle_compress(args),
        Command::Decompress(args) => handle_decompress(args),
    }
}
