use std::collections::HashSet;

use next_pwd::{Password, PasswordIterator};

mod io;

fn main() {
    let pwd: Password<8> =
        Password::from_str(io::read_password().as_str()).expect("invalid password");

    let mut pi = PasswordIterator { pwd };

    let blacklist = HashSet::from(['i', 'o', 'l']);
    let next = pi.find(|p| {
        let chars = p.value;
        if blacklist.iter().any(|c| chars.contains(c)) {
            return false;
        }
        if chars
            .windows(2)
            .fold(HashSet::new(), |mut acc, curr| {
                if curr[0] == curr[1] {
                    acc.insert(&curr[0]);
                }
                acc
            })
            .len()
            < 2
        {
            return false;
        }

        for window in chars.windows(3) {
            let left = window[0] as i32;
            let middle = window[1] as i32;
            let right = window[2] as i32;
            if right - middle == middle - left && right - left == 2 {
                return true;
            }
        }

        return false;
    });

    let next_password = next.expect("could not find next password");
    println!("{}", next_password.value.iter().collect::<String>());
}
