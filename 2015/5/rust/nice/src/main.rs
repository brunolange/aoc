use std::io::{self, BufRead};
use std::fs::File;


fn lines() -> Box<dyn Iterator<Item=String>> {
    match std::env::args().nth(1) {
        None => {
            Box::new(io::stdin().lock().lines().filter_map(Result::ok))
        },
        Some(path) => {
            let file = File::open(path).expect("error reading file");
            Box::new(io::BufReader::new(file).lines().filter_map(Result::ok))
        }
    }
}

fn main() {
    println!("{}", lines().filter(|y| is_nice_string(y)).collect::<Vec<_>>().len());
}

fn is_vowel(c: &char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn has_letter_that_appears_twice_in_a_row(text: &str) -> bool {
    text.chars()
        .zip(text.chars().skip(1))
        .any(|(curr, char)| curr == char)
}

const BLACKLIST: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn does_not_contain_blacklisted_substrings(text: &str) -> bool {
    !BLACKLIST.into_iter().any(|s| text.contains(s))
}

#[allow(unused)]
fn is_nice_string(text: &str) -> bool {
    let vowel_count = text.chars().filter(is_vowel).collect::<Vec<_>>().len();

    vowel_count >= 3
        && has_letter_that_appears_twice_in_a_row(text)
        && does_not_contain_blacklisted_substrings(text)
}

#[test]
fn test_nice_strings() {
    for nice in vec!["ugknbfddgicrmopn", "aaa"] {
        assert_eq!(is_nice_string(nice), true);
    }
}

#[test]
fn test_naughty_strings() {
    for naughty in vec!["jchzalrnumimnmhp", "haegwjzuvuyypxyu", "dvszwmarrgswjxmb"] {
        assert_eq!(is_nice_string(naughty), false);
    }
}
