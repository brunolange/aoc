use std::fs::File;
use std::io::{self, BufRead};

pub fn lines() -> Box<dyn Iterator<Item = String>> {
    let stdin_lines = || Box::new(io::stdin().lock().lines().filter_map(Result::ok));
    match std::env::args().nth(1) {
        None => stdin_lines(),
        Some(path) => {
            if path == "--" {
                return stdin_lines();
            }
            let file = File::open(path).expect("error reading file");
            Box::new(io::BufReader::new(file).lines().filter_map(Result::ok))
        }
    }
}

pub fn duration() -> usize {
    std::env::args()
        .nth(2)
        .unwrap_or("1000".to_owned())
        .parse()
        .unwrap()
}
