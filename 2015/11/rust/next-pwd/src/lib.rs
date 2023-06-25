use log::debug;
use std::collections::HashSet;

use nom::{bytes::complete::take_while_m_n, combinator::all_consuming, IResult};

#[derive(Clone, Debug)]
pub struct Password<const N: usize> {
    pub value: [char; N],
}

impl<const N: usize> Password<N> {
    pub fn from_str(s: &str) -> Option<Self> {
        let (_, pwd) = parse_pwd::<N>(s).ok()?;
        let chars: Vec<_> = pwd.chars().collect();
        let pwd = chars.try_into().ok()?;
        Some(Password { value: pwd })
    }
}

#[derive(Debug)]
pub struct PasswordIterator<const N: usize> {
    pub pwd: Password<N>,
}

impl<const N: usize> Iterator for PasswordIterator<N> {
    type Item = Password<N>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.pwd.clone().value;

        while increment(&mut chars) && flip_blacklisted(&mut chars) {
            if password_contains_3_characters_in_sequence(&chars)
                && password_contains_at_least_2_different_pairs_of_letters(&chars)
            {
                self.pwd = Password { value: chars };
                return Some(self.pwd.clone());
            }
        }

        None
    }
}

impl<const N: usize> Password<N> {
    pub fn iter(&self) -> PasswordIterator<N> {
        PasswordIterator { pwd: self.clone() }
    }
}

pub fn next_passwords(curr: &str, n: usize) -> impl Iterator<Item = String> {
    let curr_pwd: Password<8> = Password::from_str(curr).expect("Invalid password seed");

    curr_pwd
        .iter()
        .take(n)
        .map(|pwd| pwd.value.iter().collect())
        .into_iter()
}

const BLACKLIST: [char; 3] = ['i', 'o', 'l'];

fn is_blacklisted(c: &char) -> bool {
    BLACKLIST.contains(c)
}

fn password_contains_3_characters_in_sequence<const N: usize>(chars: &[char; N]) -> bool {
    return chars.windows(3).any(|window| {
        let left = window[0] as i32;
        let middle = window[1] as i32;
        let right = window[2] as i32;
        return right - middle == middle - left && right - left == 2;
    });
}

fn password_contains_at_least_2_different_pairs_of_letters<const N: usize>(
    chars: &[char; N],
) -> bool {
    return chars
        .iter()
        .zip(chars.iter().skip(1))
        .fold(HashSet::new(), |mut acc, (left, right)| {
            if *left == *right {
                acc.insert(left);
            }
            acc
        })
        .len()
        >= 2;
}

fn increment<const N: usize>(chars: &mut [char; N]) -> bool {
    let mut carry = true;
    let mut i: i32 = N as i32 - 1;
    while carry && i >= 0 {
        let idx = i as usize;
        (chars[idx], carry) = inc(chars[idx]);
        i -= 1;
    }

    !carry || i >= 0
}

fn parse_pwd<const N: usize>(input: &str) -> IResult<&str, &str> {
    let (remaining, pwd) = all_consuming(take_while_m_n(N, N, |c: char| c.is_lowercase()))(input)?;
    Ok((remaining, pwd))
}

fn inc(c: char) -> (char, bool) {
    let carry = c == 'z';
    let nxt = if carry {
        'a'
    } else {
        ((c as u32) + 1).try_into().unwrap()
    };
    (nxt, carry)
}

fn flip_blacklisted<const N: usize>(chars: &mut [char; N]) -> bool {
    debug!("checking chars: {:?}", chars);

    for i in (0..N).rev() {
        if is_blacklisted(&chars[i]) {
            debug!("found blacklisted charater: {} at index {}!", chars[i], i);
            let (ic, mut carry) = inc(chars[i]);
            chars[i] = ic;

            // propagate the carry
            let mut j = i as i32 - 1;
            while carry && j >= 0 {
                let idx = j as usize;
                (chars[idx], carry) = inc(chars[idx]);
                j -= 1;
            }

            // did we overflow?
            if carry && j < 0 {
                println!("pointless to continue...");
                return false;
            }

            // flip over the chars to the right
            for k in i + 1..N {
                chars[k] = 'a';
            }
            debug!("flipped chars = {:?}", chars);
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inc() {
        assert_eq!(inc('a'), ('b', false));
        assert_eq!(inc('l'), ('m', false));
        assert_eq!(inc('z'), ('a', true));
    }

    #[test]
    fn test_password_iterator_in_for_loop() {
        let pwd: Password<8> = Password::from_str("zzzzaaaa").unwrap();
        let mut next_pwds = vec![];
        for next_pwd in pwd.iter() {
            next_pwds.push(next_pwd);
        }

        assert_eq!(next_pwds.len(), 29);
    }

    #[test]
    fn test_flip_backlisted() {
        let mut chars = ['g', 'h', 'i', 'j', 'k', 'l', 'm', 'n'];
        let ok = flip_blacklisted(&mut chars);
        assert_eq!(chars, ['g', 'h', 'j', 'a', 'a', 'a', 'a', 'a']);
        assert!(ok);

        let mut chars = ['i', 'h', 'i', 'j', 'k', 'l', 'm', 'n'];
        let ok = flip_blacklisted(&mut chars);
        assert_eq!(chars, ['j', 'a', 'a', 'a', 'a', 'a', 'a', 'a']);
        assert!(ok);
    }
}
