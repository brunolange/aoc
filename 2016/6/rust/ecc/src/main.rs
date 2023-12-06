use std::collections::HashMap;
use std::hash::Hash;

mod io;
mod utils;

fn to_counter<T: Hash + Eq>(vec: Vec<T>) -> HashMap<T, usize> {
    let mut counter = HashMap::new();
    vec.into_iter().for_each(|c| {
        let count = counter.entry(c).or_insert(0 as usize);
        *count += 1;
    });
    counter
}

fn main() {
    let code: String = utils::transpose(
        io::lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect(),
    )
    .into_iter()
    .map(to_counter)
    .map(|count_map| {
        let (max_char, _) = count_map
            .into_iter()
            .max_by_key(|&(_, count)| count) // PART 1
            // .min_by_key(|&(_, count)| count) // PART 2
            .unwrap();
        max_char
    })
    .collect();

    println!("{code}");
}
