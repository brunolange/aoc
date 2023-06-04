use std::collections::HashSet;

use nom::{bytes::complete::take_while_m_n, combinator::all_consuming, IResult};

const BLACKLIST: [char; 3] = ['i', 'o', 'l'];

pub fn is_valid_password<const N: usize>(pwd: &Password<N>) -> bool {
    return password_does_not_contain_blacklisted_characters(pwd)
        && password_contains_3_characters_in_sequence(pwd)
        && password_contains_at_least_2_different_pairs_of_letters(pwd);
}

fn password_does_not_contain_blacklisted_characters<const N: usize>(pwd: &Password<N>) -> bool {
    let chars = pwd.value;
    return !BLACKLIST.iter().any(|c| chars.contains(c));
}

fn password_contains_at_least_2_different_pairs_of_letters<const N: usize>(
    pwd: &Password<N>,
) -> bool {
    let chars = pwd.value;
    return chars
        .windows(2)
        .fold(HashSet::new(), |mut acc, curr| {
            if curr[0] == curr[1] {
                acc.insert(&curr[0]);
            }
            acc
        })
        .len()
        >= 2;
}

fn password_contains_3_characters_in_sequence<const N: usize>(pwd: &Password<N>) -> bool {
    let chars = pwd.value;
    return chars.windows(3).any(|window| {
        let left = window[0] as i32;
        let middle = window[1] as i32;
        let right = window[2] as i32;
        return right - middle == middle - left && right - left == 2;
    });
}

pub fn parse_pwd<const N: usize>(input: &str) -> IResult<&str, &str> {
    let (remaining, pwd) = all_consuming(take_while_m_n(N, N, |c: char| c.is_lowercase()))(input)?;
    Ok((remaining, pwd))
}

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
        let mut nxt = self.pwd.clone();

        let mut carry = true;
        let mut i: i32 = N as i32 - 1;
        while carry && i >= 0 {
            let idx = i as usize;
            (nxt.value[idx], carry) = inc(nxt.value[idx]);
            i -= 1;
        }

        self.pwd = nxt.clone();

        if carry && i == -1 {
            None
        } else {
            Some(nxt)
        }
    }
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
    fn test_password_iterator() {
        let pwd: Password<3> = Password::from_str("aaa").unwrap();
        let mut pi = PasswordIterator { pwd };
        assert_eq!(pi.next().unwrap().value, ['a', 'a', 'b']);
    }
}
