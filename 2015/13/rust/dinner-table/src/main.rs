use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::{
    character::complete::{alpha1, space0},
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Debug)]
enum Mood {
    Gain,
    Lose,
}

impl FromStr for Mood {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gain" => Ok(Mood::Gain),
            "lose" => Ok(Mood::Lose),
            _ => Err(()),
        }
    }
}

fn parse_mood(input: &str) -> IResult<&str, Mood> {
    map_res(
        terminated(alt((tag("gain"), tag("lose"))), space1),
        str::parse,
    )(input)
}

#[derive(Debug)]
struct Pairing<'a> {
    first: &'a str,
    mood: Mood,
    value: usize,
    second: &'a str,
}

fn parse_line(input: &str) -> IResult<&str, Pairing> {
    let (input, first) = terminated(alpha1, space0)(input)?;
    let (input, mood) = preceded(tag("would "), parse_mood)(input)?;
    let (input, value) = map_res(digit1, |v: &str| v.parse::<usize>())(input)?;
    let (input, second) = preceded(
        tuple((space0, tag("happiness units by sitting next to "))),
        alpha1,
    )(input)?;
    Ok((
        input,
        Pairing {
            first,
            mood,
            value,
            second,
        },
    ))
}

fn main() {
    let s = "Alice would lose 79 happiness units by sitting next to Carol.";
    let x = parse_line(s);
    println!("x = {:?}", x);
}
