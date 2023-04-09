mod args;

use wrapping_paper::{line_to_box, paper_needed, GiftBox};

use std::fs::File;
use std::io::{BufRead, BufReader};

use args::Args;
use clap::Parser;

fn boxes(reader: BufReader<File>) -> impl Iterator<Item = GiftBox> {
    reader.lines().map(|line| {
        let line = line.expect("Failed to read line");
        line_to_box(&line)
    })
}

fn main() {
    let args = Args::parse();
    let file = File::open(args.input).expect("Failed to open file");
    let reader = BufReader::new(file);
    let order: u32 = boxes(reader).map(|b| paper_needed(&b)).sum();
    println!("order = {}", order);
}
