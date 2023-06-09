use std::{fs::File, io::BufRead};

pub fn lines() -> Box<dyn Iterator<Item = String>> {
    match std::env::args().nth(1) {
        None => Box::new(std::io::stdin().lock().lines().filter_map(Result::ok)),
        Some(path) => {
            let file = File::open(path).expect("Error reading file");
            Box::new(std::io::BufReader::new(file).lines().filter_map(Result::ok))
        }
    }
}

pub fn part() -> String {
    let part = std::env::var("PART").ok();
    match part {
        None => "1".to_owned(),
        Some(part) => part,
    }
}
