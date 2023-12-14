use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Animate { pause: u64 },
}

impl Cli {
    pub fn new() -> Self {
        Cli::parse()
    }
}
