mod io;
mod models;
mod parsers;

use models::Room;

fn main() {
    let output = io::lines()
        .map(|line| line.parse::<Room>().unwrap())
        .filter(|room| room.is_real())
        .inspect(|room| {
            let msg = decode(&room.name, room.sector_id);
            println!("{:?} => {}", room, msg);
        })
        .map(|room| room.sector_id)
        .sum::<usize>();

    println!("output = {}", output);
}

const LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn decode(s: &str, sector_id: usize) -> String {
    assert!(sector_id != 0, "sector_id cannot be zero");
    s.chars()
        .map(|c| {
            if c == '-' {
                return ' ';
            }
            let offset = c as usize - 97;
            let index = offset + sector_id;
            LETTERS[index % 26]
            // let step = offset + sector_id + 1;
            // ('a'..='z').cycle().take(step).last().unwrap()
        })
        .collect::<String>()
}
