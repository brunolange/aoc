use std::collections::HashSet;

use lightgrid::{GridPoint, Op};

use std::fs::File;
use std::io::{self, BufRead};

fn lines() -> Box<dyn Iterator<Item = String>> {
    match std::env::args().nth(1) {
        None => Box::new(io::stdin().lock().lines().filter_map(Result::ok)),
        Some(path) => {
            let file = File::open(path).expect("error reading file");
            Box::new(io::BufReader::new(file).lines().filter_map(Result::ok))
        }
    }
}

fn main() {
    let mut lit: HashSet<GridPoint> = HashSet::new();

    for line in lines() {
        let op = line.parse::<Op>();
        match op {
            Ok(op) => execute(&mut lit, op),
            Err(_) => panic!("Error parsing line: {}", line),
        }
    }

    println!("{}", lit.len());
}

fn execute(lit: &mut HashSet<GridPoint>, op: Op) {
    _ = match op {
        Op::Toggle(mut rect) => {
            // println!("Gotta toggle!");
            for grid_point in rect.iter() {
                if lit.contains(&grid_point) {
                    // println!("  OFF {:?}", grid_point);
                    lit.remove(&grid_point);
                } else {
                    // println!("  ON  {:?}", grid_point);
                    lit.insert(grid_point);
                }
            }
            // println!("");
        }
        Op::Turn(true, mut rect) => {
            // println!("Gotta turn on!");
            for grid_point in rect.iter() {
                // println!("  ON  {:?}", grid_point);
                lit.insert(grid_point);
            }
            // println!("");
        }
        Op::Turn(false, mut rect) => {
            // println!("Gotta turn off!");
            for grid_point in rect.iter() {
                // println!("  OFF {:?}", grid_point);
                lit.remove(&grid_point);
            }
            // println!("");
        }
    }
}
