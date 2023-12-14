use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct MyArgs {
    /// enable to show animation
    #[clap(long, default_value_t = false)]
    pub interactive: bool,

    /// interval in milliseconds to pause between animation frames. Only relevant with --interactive
    #[clap(long, default_value_t = 10)]
    pub pause: u64,
}

impl MyArgs {
    pub fn new() -> Self {
        MyArgs::parse()
    }
}
