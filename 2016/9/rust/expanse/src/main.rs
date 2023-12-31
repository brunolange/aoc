use expanse::{decoded_count, decoded_count_up_to};

mod io;

fn main() {
    let depth = io::depth();
    for line in io::lines() {
        let count = if let Some(depth) = depth {
            decoded_count_up_to(&line, depth)
        } else {
            decoded_count(&line)
        };
        println!("{count}");
    }
}
