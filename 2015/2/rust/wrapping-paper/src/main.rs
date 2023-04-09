mod args;

use wrapping_paper::{paper_needed, RectangularPrism};

use std::fs::File;
use std::io::{BufRead, BufReader};

use args::Args;
use clap::Parser;

fn rectangular_prisms(reader: BufReader<File>) -> impl Iterator<Item = RectangularPrism> {
    reader.lines().map(|line| {
        let line = line.expect("Failed to read line");
        let dimensions: Vec<u32> = line.split("x").map(|x| x.parse().unwrap()).collect();

        RectangularPrism {
            height: dimensions[0],
            length: dimensions[1],
            width: dimensions[2],
        }
    })
}

fn main() {
    let args = Args::parse();
    let file = File::open(args.input).expect("Failed to open file");
    let reader = BufReader::new(file);
    let order: u32 = rectangular_prisms(reader).map(|rp| paper_needed(&rp)).sum();
    println!("order = {}", order);
}
