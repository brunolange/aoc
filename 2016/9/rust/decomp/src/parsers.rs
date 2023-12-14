use nom::{character::complete::digit1, combinator::map_res, IResult};

pub fn parse_usize(s: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(s)
}
