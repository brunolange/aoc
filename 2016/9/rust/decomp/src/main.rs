use decomp::decompress;
use std::io::BufRead;

fn main() {
    for line in std::io::stdin().lock().lines().map_while(Result::ok) {
        println!("{}", decompress(&line).len());
    }
}
