use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct MyArgs {
    /// interval in milliseconds to pause between animation frames
    #[clap(long, default_value_t = 10)]
    pub pause: u64,
}
