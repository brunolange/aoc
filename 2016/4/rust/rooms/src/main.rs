mod io;
mod models;

use models::Room;

fn main() {
    let output = io::lines()
        .map(|line| line.parse::<Room>().unwrap())
        .filter(|room| room.is_real())
        .map(|room| room.sector_id)
        .sum::<usize>();

    println!("output = {}", output);
}
