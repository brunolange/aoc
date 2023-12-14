use tfa::Grid;

mod args;
mod io;

use args::MyArgs;

fn main() {
    let args = MyArgs::new();

    let mut grid = Grid([[false; 50]; 6]);

    if args.interactive {
        io::clear();
        io::flash(&format!("{grid}"), args.pause);
    }

    io::lines()
        .map(|line| line.parse().expect("invalid instruction"))
        .for_each(|instruction| {
            grid.apply(&instruction);
            if args.interactive {
                io::flash(&format!("{grid}"), args.pause);
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
