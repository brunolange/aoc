use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

use recipe::Ingredient;

/// Returns an iterator that yields lines either from stdin if no arguments have been passed to the CLI,
/// or lines from the file whose name is the first argument to the CLI.
pub fn lines() -> Box<dyn Iterator<Item = String>> {
    match std::env::args().nth(1) {
        None => Box::new(io::stdin().lock().lines().filter_map(Result::ok)),
        Some(path) => {
            let file = File::open(path).expect("error reading file");
            Box::new(io::BufReader::new(file).lines().filter_map(Result::ok))
        }
    }
}

pub fn parse_line(line: String) -> Ingredient {
    let parts: Vec<_> = line.split(":").collect();
    let name = parts[0].trim().to_string();
    let properties: HashMap<&str, i64> = parts[1]
        .split(",")
        .map(|v| {
            let kv = v.trim().split(" ").collect::<Vec<_>>();
            (kv[0].trim(), kv[1].trim().parse::<i64>().unwrap())
        })
        .collect();

    Ingredient {
        name,
        capacity: properties["capacity"],
        durability: properties["durability"],
        flavor: properties["flavor"],
        texture: properties["texture"],
        calories: properties["calories"] as usize,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line_parser() {
        assert_eq!(
            parse_line(
                "Sprinkles: capacity 2, durability 0, flavor -2, texture 0, calories 3".to_string(),
            ),
            Ingredient {
                name: "Sprinkles".to_string(),
                capacity: 2,
                durability: 0,
                flavor: -2,
                texture: 0,
                calories: 3
            }
        )
    }
}
