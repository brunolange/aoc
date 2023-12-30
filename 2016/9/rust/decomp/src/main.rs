use decomp::{decoded_count, decoded_count_up_to};
use std::io::BufRead;

fn main() {
    for line in std::io::stdin().lock().lines().map_while(Result::ok) {
        for max_depth in 1..10 {
            println!("{}", decoded_count_up_to(&line, max_depth));
        }
        println!();
        println!("{}", decoded_count(&line));
    }
}
