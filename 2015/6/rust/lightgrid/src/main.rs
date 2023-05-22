use std::collections::HashMap;

use lightgrid::{GridPoint, Op};
use log::warn;

mod io;

fn main() {
    env_logger::init();
    let mut brightness_map: HashMap<GridPoint, usize> = HashMap::new();

    for (i, line) in crate::io::lines().enumerate() {
        let op = line.parse::<Op>();
        match op {
            Ok(op) => execute(&mut brightness_map, op),
            Err(_) => {
                warn!("Ignoring line {}: [{}]", i + 1, line);
            }
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
