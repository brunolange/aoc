use std::collections::HashMap;

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
    let mut brightness_map: HashMap<GridPoint, usize> = HashMap::new();

    for line in lines() {
        let op = line.parse::<Op>();
        match op {
            Ok(op) => execute(&mut brightness_map, op),
            Err(_) => panic!("Error parsing line: {}", line),
        }
    }

    let total_brightness: usize = brightness_map.values().sum();
    println!("{}", total_brightness);
}

fn execute(brightness_map: &mut HashMap<GridPoint, usize>, op: Op) {
    match op {
        Op::Toggle(mut rect) => {
            for grid_point in rect.iter() {
                *brightness_map.entry(grid_point).or_insert(0) += 2;
            }
        }
        Op::Turn(true, mut rect) => {
            for grid_point in rect.iter() {
                *brightness_map.entry(grid_point).or_insert(0) += 1;
            }
        }
        Op::Turn(false, mut rect) => {
            for grid_point in rect.iter() {
                let _ = *brightness_map
                    .entry(grid_point)
                    .and_modify(|v| {
                        if *v == 0 {
                            return;
                        }
                        *v -= 1;
                    })
                    .or_insert(0);
            }
        }
    }
}
