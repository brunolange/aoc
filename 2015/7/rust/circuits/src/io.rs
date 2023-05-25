use std::env::VarError;
use std::fs::File;
use std::io::{self, BufRead};

use circuits::models::SignalMap;

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

pub fn wire() -> Result<String, VarError> {
    std::env::var("WIRE")
}

#[derive(Debug)]
pub enum Output {
    SingleWire(u16),
    AllWires(SignalMap),
    Error,
}
