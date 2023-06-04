use log::debug;
use std::collections::HashSet;

use nom::{bytes::complete::take_while_m_n, combinator::all_consuming, IResult};

const BLACKLIST: [char; 3] = ['i', 'o', 'l'];

pub fn next_password(curr: &str) -> Option<String> {
    let pwd: Password<8> = Password::from_str(curr).expect("invalid password to begin with");

    let mut pi = PasswordIterator { pwd };

    pi.find(is_valid_password).map(|p| p.value.iter().collect())
}

pub fn next_password_fast(curr: &str) -> Option<String> {
    let pwd: Password<8> = Password::from_str(curr).expect("invalid password to begin with");

    let mut pi = FastPasswordIterator { pwd };

    pi.next().map(|p| p.value.iter().collect())
}

fn is_valid_password<const N: usize>(pwd: &Password<N>) -> bool {
    debug!(
        "Checking password: {:?}",
        pwd.value.iter().collect::<String>()
    );
    let chars = pwd.value;
    return password_does_not_contain_blacklisted_characters(&chars)
        && password_contains_3_characters_in_sequence(&chars)
        && password_contains_at_least_2_different_pairs_of_letters(&chars);
}

fn password_does_not_contain_blacklisted_characters<const N: usize>(chars: &[char; N]) -> bool {
    !chars.iter().any(is_blacklisted)
}

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

#[derive(Clone, Debug)]
struct Password<const N: usize> {
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

fn parse_pwd<const N: usize>(input: &str) -> IResult<&str, &str> {
    let (remaining, pwd) = all_consuming(take_while_m_n(N, N, |c: char| c.is_lowercase()))(input)?;
    Ok((remaining, pwd))
}

#[derive(Debug)]
struct PasswordIterator<const N: usize> {
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

#[derive(Debug)]
struct FastPasswordIterator<const N: usize> {
    pub pwd: Password<N>,
}

impl<const N: usize> Iterator for FastPasswordIterator<N> {
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
    fn test_password_iterator() {
        let pwd: Password<3> = Password::from_str("aaa").unwrap();
        let mut pi = PasswordIterator { pwd };
        assert_eq!(pi.next().unwrap().value, ['a', 'a', 'b']);
    }

    #[test]
    fn test_next_password() {
        assert_eq!(next_password("abcdefgh"), Some(String::from("abcdffaa")));
        assert_eq!(next_password("ghijklmn"), Some(String::from("ghjaabcc")));
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

    #[test]
    fn test_password_fast_iterator() {
        assert_eq!(
            next_password_fast("abcdefgh"),
            Some(String::from("abcdffaa"))
        );
        assert_eq!(
            next_password_fast("ghijklmn"),
            Some(String::from("ghjaabcc"))
        );
    }
}
