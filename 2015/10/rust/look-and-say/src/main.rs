use itertools::Itertools;

fn look_and_say(input: String) -> String {
    input
        .chars()
        .group_by(|c| *c)
        .into_iter()
        .map(|(c, grp)| format!("{}{}", grp.collect::<Vec<_>>().len(), c))
        .join("")
}

fn main() {
    let seed = std::env::args().nth(1).unwrap_or("1113122113".to_owned());
    let iterations: usize = std::env::args()
        .nth(2)
        .unwrap_or("40".to_owned())
        .parse()
        .unwrap();

    let mut last = seed;
    for _ in 0..iterations {
        last = look_and_say(last);
    }

    println!("{}: {}", last, last.len());
}
