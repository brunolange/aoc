use nom::{character::complete::digit1, combinator::map_res, error::ParseError, IResult, Parser};

pub fn parse_usize(s: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(s)
}

pub fn take_anything_until<'a, F, O, E>(
    mut f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, (&'a str, O), E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, O, E>,
{
    move |input: &str| {
        let len = input.len();
        let mut index = 0;
        loop {
            match f.parse(&input[index..]) {
                Ok((i1, o)) => return Ok((i1, (&input[..index], o))),
                Err(nom::Err::Error(err)) => {
                    index += 1;
                    if index >= len {
                        return Err(nom::Err::Error(err));
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }
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

    #[test]
    fn test_take_till() {
        let (s, (prefix, x)) = take_anything_until(parse_usize)("hello 42!").unwrap();
        assert_eq!(s, "!");
        assert_eq!(prefix, "hello ");
        assert_eq!(x, 42);

        let output = take_anything_until(parse_usize)("hello world!");
        assert!(output.is_err());
    }
}
