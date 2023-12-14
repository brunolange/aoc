use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Animate(Animation),
}

#[derive(Debug, Args)]
pub struct Animation {
    /// interval in milliseconds to pause between animation frames.
    #[clap(long, default_value_t = 10)]
    pub pause: u64,
}

impl Cli {
    pub fn new() -> Self {
        Cli::parse()
    }
}
