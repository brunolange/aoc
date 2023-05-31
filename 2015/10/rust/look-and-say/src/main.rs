use itertools::Itertools;

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
    let seed = std::env::args().nth(1).unwrap_or("1113122113".to_owned());
    let iterations: usize = std::env::args()
        .nth(2)
        .unwrap_or("40".to_owned())
        .parse()
        .unwrap();
    let las = LookAndSay { curr: seed };
    let last = las.take(iterations).last().unwrap();
    println!("{}: {}", last, last.len());
}
