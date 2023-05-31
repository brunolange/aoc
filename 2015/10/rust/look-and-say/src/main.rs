use itertools::Itertools;

struct LookAndSay {
    seed: String,
}

impl LookAndSay {
    fn iter(&mut self) -> LookAndSayIter {
        LookAndSayIter {
            curr: self.seed.clone(),
        }
    }
}

struct LookAndSayIter {
    curr: String,
}

impl Iterator for LookAndSayIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let response = Some(self.curr.clone());
        self.curr = look_and_say(self.curr.clone());
        response
    }
}

fn look_and_say(input: String) -> String {
    input
        .chars()
        .group_by(|c| *c)
        .into_iter()
        .map(|(c, grp)| (grp.collect::<Vec<_>>().len().to_string(), c))
        .map(|(count, c)| format!("{}{}", count, c))
        .join("")
}

fn main() {
    let mut las = LookAndSay {
        seed: "1113122113".to_owned(),
    };
    let last = las.iter().take(41).last().unwrap();
    println!("{}", last.len());
}
