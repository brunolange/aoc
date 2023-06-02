mod io;

use io::lines;

use matchsticks::{counts, encode_counts};

fn main() {
    let mapper = match io::part() {
        io::Part::ONE => counts,
        io::Part::TWO => encode_counts,
    };

    println!(
        "{}",
        lines()
            .map(|line| mapper(&line))
            .fold(0, |acc, (l, r)| acc + l - r)
    );
}
