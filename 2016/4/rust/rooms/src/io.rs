use std::fs::File;
use std::io::{self, BufRead};

pub fn lines() -> Box<dyn Iterator<Item = String>> {
    match std::env::args().nth(1) {
        None => Box::new(
            io::stdin()
                .lock()
                .lines()
                .map_while(Result::ok)
                .filter(|line| !line.starts_with("--")),
        ),
        Some(path) => {
            let file = File::open(path).expect("error reading file");
            Box::new(io::BufReader::new(file).lines().map_while(Result::ok))
        }
    }
}
