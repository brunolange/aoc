use std::collections::HashSet;

use itertools::Itertools;
use md5;

struct PasswordCracker {
    door_id: String,
    curr_index: i32,
}

impl PasswordCracker {
    fn new(door_id: String) -> Self {
        PasswordCracker {
            door_id,
            curr_index: 0,
        }
    }
}

impl Iterator for PasswordCracker {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        for index in self.curr_index.. {
            let s = format!("{}{}", self.door_id, index);
            let hash = format!("{:x}", md5::compute(s));
            if hash.starts_with("00000") {
                let c = hash.chars().nth(5).unwrap();
                self.curr_index = index + 1;
                return Some(c);
            }
        }
        None
    }
}

fn crack_password(door_id: &str) -> String {
    PasswordCracker::new(door_id.to_string()).take(8).collect()
}

fn crack_password_2(door_id: &str) -> Option<String> {
    let mut password = [' '; 8];
    let mut pending: HashSet<_> = (0..8).collect();
    for index in 0.. {
        let s = format!("{}{}", door_id, index);
        let hash = format!("{:x}", md5::compute(s.clone()));
        if !hash.starts_with("00000") {
            continue;
        }
        let (position, c) = hash.chars().skip(5).take(2).next_tuple().unwrap();
        if let Ok(position) = position.to_string().parse::<usize>() {
            if position >= 8 {
                continue;
            }
            if pending.contains(&position) {
                password[position] = c;
                pending.remove(&position);
            }
            if pending.len() == 0 {
                return Some(password.iter().collect());
            }
        }
    }
    None
}

fn main() {
    let input = std::env::args().nth(1).expect("need input");
    let part = std::env::var("PART").unwrap_or("1".to_string());
    let password = match part.as_str() {
        "2" => crack_password_2(&input).unwrap(),
        _ => crack_password(&input),
    };
    println!("{}", password);
}
