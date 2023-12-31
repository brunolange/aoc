use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// decompression depth
    #[clap(long)]
    pub depth: Option<usize>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Count,
    Decompress,
}
