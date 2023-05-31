struct LookAndSay<'a> {
    seed: &'a str,
}

impl LookAndSay<'_> {
    fn iter(&mut self) -> LookAndSayIter {
        LookAndSayIter { curr: self.seed }
    }
}

struct LookAndSayIter<'a> {
    curr: &'a str,
}

impl<'a> Iterator for LookAndSayIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let response = Some(self.curr);
        self.curr = "and then";
        response
    }
}

fn main() {
    let mut i = 0;
    let mut las = LookAndSay { seed: "hello" };
    for s in las.iter() {
        println!("s = {}", s);
        i += 1;
        if i == 5 {
            break;
        }
    }
    println!("Hello, world!");
}
