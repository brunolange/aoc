use std::fs::read_to_string;

type G = Vec<Vec<bool>>;

fn parse_grid(s: &str) -> G {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("invalid character"),
                })
                .collect()
        })
        .collect()
}

fn main() {
    let contents = read_to_string("input.txt").unwrap();
    let grid = parse_grid(&contents);
    println!("{:?}", grid[0]);
}
