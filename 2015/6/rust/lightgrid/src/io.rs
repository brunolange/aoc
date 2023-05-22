use std::fs::File;
use std::io::{self, BufRead};

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
