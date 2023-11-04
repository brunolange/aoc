mod io;
mod models;

use models::{decode_lines, Button};

fn main() {
    decode_lines(Button::Five, io::lines()).for_each(|button| println!("{:?}", button));
}
