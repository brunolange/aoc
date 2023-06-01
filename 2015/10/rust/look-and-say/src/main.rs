use itertools::Itertools;

mod io;

fn look_and_say(input: String) -> String {
    input
        .chars()
        .group_by(|c| *c)
        .into_iter()
        .map(|(c, grp)| format!("{}{}", grp.collect::<Vec<_>>().len(), c))
        .join("")
}

fn main() {
    let mut last = io::seed();
    for _ in 0..io::iterations() {
        last = look_and_say(last);
    }

    println!("{}: {}", last, last.len());
}
