use crate::Marker;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    sequence::{delimited, separated_pair},
    IResult,
};

pub fn parse_usize(s: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(s)
}

pub fn parse_marker(s: &str) -> IResult<&str, Marker> {
    let (s, (take, repeat)) = delimited(
        tag("("),
        separated_pair(parse_usize, tag("x"), parse_usize),
        tag(")"),
    )(s)?;

    Ok((s, Marker { take, repeat }))
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
