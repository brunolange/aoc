mod args;

use gifts::{line_to_box, Order, GiftBox};

use std::fs::File;
use std::io::{BufRead, BufReader};

use args::Args;
use clap::Parser;

fn boxes(reader: BufReader<File>) -> impl Iterator<Item = Result<GiftBox, String>> {
    reader.lines().map(|line| {
        let l = line.map_err(|e| e.to_string())?;
        line_to_box(&l)
    })
}

fn main() {
    let args = Args::parse();
    let file = File::open(args.input).expect("Failed to open file");
    let reader = BufReader::new(file);
    let order: Order = boxes(reader)
        .map(|b| match b {
            Ok(b) => Order::from_gift_box(&b),
            Err(s) => {
                println!("Skipping line: {}", s);
                Order {
                    wrapping_paper: 0,
                    ribbon: 0,
                }
            }
        })
        .sum();
    println!("order = {:?}", order);
}
