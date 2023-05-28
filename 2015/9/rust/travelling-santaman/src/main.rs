mod io;

use crate::io::lines;

fn main() {
    for line in lines() {
        println!("{}", line);
    }
}
