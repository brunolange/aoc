use std::{fs::File, io::BufRead};

pub fn lines() -> Box<dyn Iterator<Item = String>> {
    match std::env::args().nth(1) {
        None => Box::new(std::io::stdin().lock().lines().map_while(Result::ok)),
        Some(path) => {
            let file = File::open(path).expect("Error reading file");
            Box::new(std::io::BufReader::new(file).lines().map_while(Result::ok))
        }
    }
}

pub enum Part {
    One,
    Two,
}

pub fn part() -> Part {
    if let Ok(part) = std::env::var("PART") {
        return match part.as_str() {
            "1" => Part::One,
            "2" => Part::Two,
            _ => panic!("invalid part"),
        };
    }
    Part::One
}
