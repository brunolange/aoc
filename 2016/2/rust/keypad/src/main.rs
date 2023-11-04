mod io;
mod models;

use models::{decode, Button};

fn main() {
    decode(Button::Five, io::lines()).for_each(|button| println!("{:?}", button));
}
