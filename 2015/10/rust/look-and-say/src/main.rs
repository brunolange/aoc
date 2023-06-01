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

struct LookAndSay {
    curr: String,
}

impl Iterator for LookAndSay {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr = look_and_say(self.curr.clone());
        Some(self.curr.clone())
    }
}

fn main() {
    let las = LookAndSay { curr: io::seed() };
    let last = las.take(io::iterations()).last().unwrap();
    println!("{}: {}", last, last.len());
}
