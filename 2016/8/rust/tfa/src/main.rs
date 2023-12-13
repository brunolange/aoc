use std::io::BufRead;

use tfa::{Grid, Instruction};

fn main() {
    let mut grid = Grid([[false; 50]; 6]);

    std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .for_each(|line| {
            let instruction: Instruction = line.parse().expect("invalid instruction");
            grid.apply(&instruction);
        });

    println!("{grid}");

    let count = grid
        .0
        .into_iter()
        .flat_map(|row| row.into_iter().filter(|c| *c))
        .count();

    println!("{count}");
}
