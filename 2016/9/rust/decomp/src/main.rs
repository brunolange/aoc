use decomp::{decoded_count, decoded_count_up_to};
use std::io::BufRead;

fn main() {
    for line in std::io::stdin().lock().lines().map_while(Result::ok) {
        if std::env::var("SHOW_PROGRESS").is_ok() {
            (1..)
                .map(|depth| {
                    (
                        decoded_count_up_to(&line, depth),
                        decoded_count_up_to(&line, depth + 1),
                    )
                })
                .enumerate()
                .find(|(index, (curr, next))| {
                    println!("{index}: {curr}");
                    curr == next
                })
                .unwrap();
        } else {
            println!("{}", decoded_count(&line));
        }
    }
}
