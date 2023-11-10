use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::IResult;

pub fn parse_name(s: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag("-"), alpha1)(s)
}

pub fn parse_usize(s: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(s)
}

pub fn parse_checksum(s: &str) -> IResult<&str, &str> {
    delimited(tag("["), alpha1, tag("]"))(s)
}
