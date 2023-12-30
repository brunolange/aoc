use nom::{character::complete::digit1, combinator::map_res, error::ParseError, IResult, Parser};

pub fn parse_usize(s: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_usize() {
        let (_, x) = parse_usize("42").unwrap();
        assert_eq!(x, 42);

        assert_eq!(parse_usize("42abc").unwrap(), ("abc", 42));
    }
}
