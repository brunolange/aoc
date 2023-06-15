use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::combinator::{map, map_res};
use nom::sequence::tuple;
use nom::{
    character::complete::{alpha1, space0},
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Debug)]
struct Pairing<'a> {
    first: &'a str,
    second: &'a str,
    gain: i32,
}

fn parse_line(input: &str) -> IResult<&str, Pairing> {
    let (input, first) = terminated(alpha1, space0)(input)?;
    let (input, mult) = preceded(
        tuple((tag("would "), space0)),
        terminated(
            alt((map(tag("gain"), |_| 1), map(tag("lose"), |_| -1))),
            space1,
        ),
    )(input)?;

    let (input, value) = map_res(digit1, |v: &str| v.parse::<i32>())(input)?;
    let (input, second) = preceded(
        tuple((space0, tag("happiness units by sitting next to "))),
        alpha1,
    )(input)?;
    Ok((
        input,
        Pairing {
            first,
            second,
            gain: mult * value,
        },
    ))
}

fn main() {
    let s = "Alice would lose 79 happiness units by sitting next to Carol.";
    let x = parse_line(s);
    println!("x = {:?}", x);

    let s = "Alice would gain 54 happiness units by sitting next to Bob.";
    let x = parse_line(s);
    println!("x = {:?}", x);
}
