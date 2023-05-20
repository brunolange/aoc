use nom::bytes::complete::take_till1;
use nom::character::complete::{digit1, space0};
use nom::combinator::{map_res, recognize};
use nom::multi::count;
use nom::sequence::preceded;
use nom::IResult;

pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

pub fn take_word(input: &str) -> IResult<&str, &str> {
    preceded(space0, recognize(take_till1(|c| c == ' ')))(input)
}

#[allow(unused)]
pub fn take_words<const N: usize>(input: &str) -> IResult<&str, [&str; N]> {
    map_res(count(take_word, N), |words| words.try_into())(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_word() {
        assert_eq!(take_word("hello").unwrap(), ("", "hello"));
        assert_eq!(take_word("hello ").unwrap(), (" ", "hello"));
        assert_eq!(take_word(" hello").unwrap(), ("", "hello"));
        assert_eq!(take_word(" hello ").unwrap(), (" ", "hello"));
        assert_eq!(take_word("hello world").unwrap(), (" world", "hello"));
        assert_eq!(take_word(" hello world").unwrap(), (" world", "hello"));
        assert_eq!(take_word("  hello world").unwrap(), (" world", "hello"));
    }

    #[test]
    fn test_take_words() {
        assert_eq!(take_words::<1>("hello").unwrap(), ("", ["hello"]));
        assert_eq!(
            take_words::<1>("hello world").unwrap(),
            (" world", ["hello"])
        );
        assert_eq!(
            take_words::<2>("hello world").unwrap(),
            ("", ["hello", "world"])
        );
        assert_eq!(
            take_words::<2>("hello hello world").unwrap(),
            (" world", ["hello", "hello"])
        );
        assert_eq!(
            take_words::<3>("hello hello world").unwrap(),
            ("", ["hello", "hello", "world"])
        );
        assert_eq!(
            take_words::<3>("    hello hello world").unwrap(),
            ("", ["hello", "hello", "world"])
        );
        assert_eq!(
            take_words::<3>("    hello hello world ").unwrap(),
            (" ", ["hello", "hello", "world"])
        );
        assert!(take_words::<1>("").is_err());
        assert!(take_words::<1>("   ").is_err());
        assert!(take_words::<2>("hello").is_err());
        assert!(take_words::<2>("hello  ").is_err());
        assert!(take_words::<2>("   hello").is_err());
        assert!(take_words::<2>("   hello   ").is_err());
    }
}
