mod io;
mod models;

use models::{decode, Button, Instruction, Sequence};

pub fn to_sequence(s: String) -> Sequence {
    s.chars()
        .map(|c| c.to_string().parse::<Instruction>().unwrap())
        .collect()
}

fn main() {
    decode(Button::Five, io::lines().map(to_sequence)).for_each(|button| println!("{:?}", button));
}
