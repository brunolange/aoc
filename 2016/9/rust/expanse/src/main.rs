use clap::Parser;
use expanse::{decoded_count, decoded_count_up_to, decompress, decompress_up_to};

mod args;
mod io;

use args::Command;

fn main() {
    let cli = args::Cli::parse();
    let depth = cli.depth;
    for line in io::lines() {
        match cli.command {
            Command::Decompress => {
                let decompressed_line = if let Some(depth) = depth {
                    decompress_up_to(&line, depth)
                } else {
                    decompress(&line)
                };
                println!("{decompressed_line}");
            }
            Command::Count => {
                let count = if let Some(depth) = depth {
                    decoded_count_up_to(&line, depth)
                } else {
                    decoded_count(&line)
                };
                println!("{count}");
            }
        }
    }
}
