use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::alphanumeric1;
use nom::combinator::all_consuming;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;

pub fn parse_count(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("\"")(input)?;
    let (input, xs) = many0(alt((parse_hex, parse_quote, parse_backslash, parse_seq)))(input)?;
    let _ = all_consuming(tag("\""))(input)?;

    Ok((input, xs.iter().sum()))
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn parse_hex(input: &str) -> IResult<&str, usize> {
    let (remaining, _) = preceded(tag(r"\x"), take_while_m_n(2, 2, is_hex_digit))(input)?;
    Ok((remaining, 1))
}

fn parse_quote(input: &str) -> IResult<&str, usize> {
    let (remaining, _) = tag("\\\"")(input)?;
    Ok((remaining, 1))
}

fn parse_backslash(input: &str) -> IResult<&str, usize> {
    let (remaining, _) = tag(r"\\")(input)?;
    Ok((remaining, 1))
}

fn parse_seq(input: &str) -> IResult<&str, usize> {
    let (remaining, seq) = alphanumeric1(input)?;
    Ok((remaining, seq.len()))
}
