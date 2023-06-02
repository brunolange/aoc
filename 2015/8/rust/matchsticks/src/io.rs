use std::fs::File;
use std::io::{self, BufRead};

pub fn lines() -> Box<dyn Iterator<Item = String>> {
    match std::env::args().nth(1) {
        None => Box::new(
            io::stdin()
                .lock()
                .lines()
                .filter_map(Result::ok)
                .filter(|line| !line.starts_with("--")),
        ),
        Some(path) => {
            let file = File::open(path).expect("error reading file");
            Box::new(io::BufReader::new(file).lines().filter_map(Result::ok))
        }
    }
}

pub enum Part {
    ONE,
    TWO,
}

pub fn part() -> Part {
    let part = std::env::var("PART")
        .unwrap_or("1".to_owned())
        .parse()
        .expect("invalid PART");

    match part {
        1 => Part::ONE,
        2 => Part::TWO,
        _ => panic!("invalid PART"),
    }
}
