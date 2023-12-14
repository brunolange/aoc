use nom::{
    bytes::complete::tag,
    sequence::{delimited, separated_pair},
    IResult,
};
use std::str::FromStr;

mod parsers;

use parsers::parse_usize;

#[derive(Debug, PartialEq, Eq)]
pub struct Marker {
    pub take: usize,
    pub repeat: usize,
}

#[derive(Debug)]
pub struct MarkerParsingError(String);

impl FromStr for Marker {
    type Err = MarkerParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, marker) = parser_marker(s)
            .map_err(|_| MarkerParsingError("detailed error goes here".to_string()))?;
        Ok(marker)
    }
}

fn parser_marker(s: &str) -> IResult<&str, Marker> {
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
    fn test_marker_parser() {
        assert_eq!(
            "(10x2)".parse::<Marker>().unwrap(),
            Marker {
                take: 10,
                repeat: 2
            }
        )
    }
}
