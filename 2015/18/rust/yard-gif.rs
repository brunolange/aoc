use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
struct Grid {
    g: Vec<Vec<bool>>,
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<bool>> = s
            .lines()
            .map(|line| {
                let row: Vec<bool> = line
                    .chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("bigode"),
                    })
                    .collect();
                row
            })
            .collect();
        Ok(Grid { g: grid })
    }
}

fn main() {
    let contents = read_to_string("input.txt").unwrap();
    let grid: Grid = contents.parse().unwrap();
    println!("{:?}", grid.g[0]);
}
