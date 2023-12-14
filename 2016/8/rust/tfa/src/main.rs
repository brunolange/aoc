use tfa::Grid;

mod io;

fn main() {
    let mut grid = Grid([[false; 50]; 6]);

    io::clear();
    io::flash(&format!("{grid}"), 1000);

    io::lines()
        .map(|line| line.parse().expect("invalid instruction"))
        .for_each(|instruction| {
            io::flash(&format!("{grid}"), 10);
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
