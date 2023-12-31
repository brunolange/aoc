use std::io::BufRead;

pub fn lines() -> impl Iterator<Item = String> {
    std::io::stdin().lock().lines().map_while(Result::ok)
}

pub fn depth() -> Option<usize> {
    std::env::var("DEPTH")
        .map(|v| v.parse::<usize>().unwrap())
        .ok()
}
