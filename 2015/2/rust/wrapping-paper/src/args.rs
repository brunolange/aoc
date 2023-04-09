use std::path::PathBuf;

use clap::Parser;

/// Simple program to calculate the amount of gift wrapping paper needed by Santa's elves
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// File containing the gifts dimensions
    pub input: PathBuf,
}
