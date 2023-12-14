use clap::Parser;
use tfa::Grid;

mod args;
mod io;

fn main() {
    let args = args::MyArgs::parse();

    let mut grid = Grid([[false; 50]; 6]);

    io::clear();
    io::flash(&format!("{grid}"), args.pause);

    io::lines()
        .map(|line| line.parse().expect("invalid instruction"))
        .for_each(|instruction| {
            grid.apply(&instruction);
            io::flash(&format!("{grid}"), args.pause);
        });

    println!("{grid}");

    let count = grid
        .0
        .into_iter()
        .flat_map(|row| row.into_iter().filter(|c| *c))
        .count();

    println!("{count}");
}
