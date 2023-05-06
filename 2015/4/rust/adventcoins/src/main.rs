mod args;

use std::time::Instant;

use adventcoins::{nonce, nonce_par};
use args::{Args, Strategy};
use clap::Parser;

fn main() {
    let args = Args::parse();
    println!("args = {:?}", args);
    let zeros = "0".repeat(args.leading_zeros);
    let solver = match args.strategy {
        Some(Strategy::Parallel) => nonce_par,
        _ => nonce
    };
    let start = Instant::now();
    let value = solver(&args.message, &zeros);
    let duration = start.elapsed();

    println!("value = {:?}", value);
    println!("Elapsed time = {:?}", duration);
}
