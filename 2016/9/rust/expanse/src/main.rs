use clap::Parser;

mod args;
mod io;

use args::Command;

fn main() {
    let cli = args::Cli::parse();

    for line in io::lines() {
        match cli.command {
            Command::Decompress => {
                println!("{}", decompress(cli.depth)(&line));
            }
            Command::Count => {
                println!("{}", count(cli.depth)(&line));
            }
        }
    }
}

fn decompress(depth: Option<usize>) -> impl Fn(&str) -> String {
    move |s: &str| {
        if let Some(depth) = depth {
            expanse::decompress_up_to(s, depth)
        } else {
            expanse::decompress(s)
        }
    }
}

fn count(depth: Option<usize>) -> impl Fn(&str) -> usize {
    move |s: &str| {
        if let Some(depth) = depth {
            expanse::decoded_count_up_to(s, depth)
        } else {
            expanse::decoded_count(s)
        }
    }
}
