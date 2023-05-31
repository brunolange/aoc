use itertools::Itertools;

fn look_and_say(input: String) -> String {
    input
        .chars()
        .group_by(|c| *c)
        .into_iter()
        .map(|(c, grp)| (grp.collect::<Vec<_>>().len().to_string(), c))
        .map(|(count, c)| format!("{}{}", count, c))
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
    let las = LookAndSay {
        curr: "1113122113".to_owned(),
    };
    let last = las.take(40).last().unwrap();
    println!("{}: {}", last, last.len());
}
