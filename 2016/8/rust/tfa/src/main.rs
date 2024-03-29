use tfa::Grid;

mod args;
mod io;

use args::Cli;

fn main() {
    let args = Cli::new();

    let mut grid = Grid([[false; 50]; 6]);

    let pause = match args.command {
        Some(args::Command::Animate(args::Animation { pause })) => {
            io::clear();
            io::flash(&format!("{grid}"), pause);
            Some(pause)
        }
        _ => None,
    };

    io::lines()
        .map(|line| line.parse().expect("invalid instruction"))
        .for_each(|instruction| {
            grid.apply(&instruction);
            if let Some(duration) = pause {
                io::flash(&format!("{grid}"), duration);
            }
        });

    println!("{grid}");

    let count = grid
        .0
        .into_iter()
        .flat_map(|row| row.into_iter().filter(|c| *c))
        .count();

    println!("{count}");
}
