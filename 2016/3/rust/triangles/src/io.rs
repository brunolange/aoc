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

pub fn read(s: String) -> (usize, usize, usize) {
    let sides: [usize; 3] = s
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        // .take(3)
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    (sides[0], sides[1], sides[2])
}
