use std::collections::HashSet;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::alphanumeric1;
use nom::combinator::all_consuming;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;

mod io;

use io::lines;

fn encode_counts(input: &str) -> (usize, usize) {
    let to_escape = HashSet::from(['"', '\\']);
    let extra = input.chars().filter(|c| to_escape.contains(c)).count();
    return (input.len() + 2 + extra, input.len());
}

fn main() {
    let x = std::env::var("PART").unwrap_or("1".to_owned());
    let mapper = match x.as_str() {
        "1" => counts,
        "2" => encode_counts,
        _ => panic!("Invalid PART"),
    };

    println!(
        "{}",
        lines()
            .map(|line| mapper(&line))
            .fold(0, |acc, (l, r)| acc + l - r)
    );
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
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

fn parse_count(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("\"")(input)?;
    let (input, xs) = many0(alt((parse_hex, parse_quote, parse_backslash, parse_seq)))(input)?;
    let _ = all_consuming(tag("\""))(input)?;

    Ok((input, xs.iter().sum()))
}

fn counts(line: &str) -> (usize, usize) {
    let (_, count) = parse_count(line).expect("Invalid input");
    (line.len(), count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts() {
        assert_eq!(counts("\"\""), (2, 0));
        assert_eq!(counts("\"abc\""), (5, 3));
        assert_eq!(counts("\"aaa\\\"aaa\""), (10, 7));
        assert_eq!(counts("\"\\x27\""), (6, 1));
    }
}
