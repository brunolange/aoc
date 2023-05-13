use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn lines() -> Box<dyn Iterator<Item = String>> {
    match std::env::args().nth(1) {
        None => Box::new(io::stdin().lock().lines().filter_map(Result::ok)),
        Some(path) => {
            let file = File::open(path).expect("error reading file");
            Box::new(io::BufReader::new(file).lines().filter_map(Result::ok))
        }
    }
}

fn main() {
    println!(
        "{}",
        lines()
            .filter(|y| is_nice_string(y))
            .collect::<Vec<_>>()
            .len()
    );
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

fn is_nice_string(text: &str) -> bool {
    let vowel_count = text.chars().filter(is_vowel).collect::<Vec<_>>().len();

    vowel_count >= 3
        && has_letter_that_appears_twice_in_a_row(text)
        && does_not_contain_blacklisted_substrings(text)
}

#[allow(unused)]
fn is_nice_string_2(text: &str) -> bool {
    has_two_pairs_with_no_overlapping(text) && has_letter_sandwich(text)
}

fn has_two_pairs_with_no_overlapping(txt: &str) -> bool {
    let pairs = txt.chars().zip(txt.chars().skip(1));
    let pair_map: HashMap<(char, char), Vec<usize>> = HashMap::new();
    let indices_map = pairs.enumerate().fold(pair_map, |mut acc, curr| {
        let (index, pair) = curr;
        let s = acc.entry(pair).or_insert(Vec::new());
        s.push(index);
        acc
    });

    indices_map.iter().any(|(_, indices)| {
        indices.len() > 2
            || indices
                .iter()
                .zip(indices.iter().skip(1))
                .any(|(curr, succ)| curr.abs_diff(*succ) > 1)
    })
}

fn has_letter_sandwich(text: &str) -> bool {
    text.chars().zip(text.chars().skip(1).zip(text.chars().skip(2))).any(|(left, (_, right))| {
        left == right
    })
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

#[test]
fn test_nice_strings2() {
    for nice in vec!["qjhvhtzxzqqjkmpb", "xxyxx"] {
        assert!(is_nice_string_2(nice));
    }
}

#[test]
fn test_naughty_strings_2() {
    for naughty in vec!["uurcxstgmygtbstg", "ieodomkazucvgmuy"] {
        assert_eq!(is_nice_string_2(naughty), false);
    }
}
