mod password;

use password::{is_valid_password, FastPasswordIterator, Password, PasswordIterator};

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_password() {
        assert_eq!(next_password("abcdefgh"), Some(String::from("abcdffaa")));
        assert_eq!(next_password("ghijklmn"), Some(String::from("ghjaabcc")));
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
