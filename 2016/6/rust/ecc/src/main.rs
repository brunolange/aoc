use std::collections::HashMap;

mod io;
mod utils;

fn main() {
    let code: String = utils::transpose(
        io::lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect(),
    )
    .into_iter()
    .map(|column| {
        let mut count_map = HashMap::new();
        column.into_iter().for_each(|c| {
            let count = count_map.entry(c).or_insert(0 as usize);
            *count += 1;
        });
        count_map
    })
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
