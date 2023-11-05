use itertools::Itertools;
use triangles::{can_make_triangle, vertice};

mod io;
mod utils;

fn main() {
    let lines = io::lines();

    let count = match io::part() {
        io::Part::One => lines
            .map(io::read_k::<3>)
            .filter(|[a, b, c]| can_make_triangle(*a, *b, *c))
            .map(|[a, b, c]| {
                println!("{}, {}, {} => {:?}", a, b, c, vertice(a, b, c));
            })
            .count(),

        io::Part::Two => lines
            .map(io::read_n)
            .chunks(3)
            .into_iter()
            .map(|rows| rows.collect::<Vec<Vec<usize>>>())
            .flat_map(utils::transpose)
            .filter(|sides| {
                let [a, b, c]: [usize; 3] = sides[..]
                    .try_into()
                    .unwrap_or_else(|_| panic!("Needed 3 lines but only got {}", sides.len()));
                can_make_triangle(a, b, c)
            })
            .count(),
    };

    println!("count = {count}");
}
