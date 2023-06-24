mod password;

use password::Password;

pub fn next_password(curr: &str) -> Option<String> {
    let pwd: Password<8> = Password::from_str(curr).expect("invalid password to begin with");

    pwd.iter().next().map(|p| p.value.iter().collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_password() {
        assert_eq!(next_password("abcdefgh"), Some(String::from("abcdffaa")));
        assert_eq!(next_password("ghijklmn"), Some(String::from("ghjaabcc")));
    }
}
