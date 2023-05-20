use std::str::FromStr;

use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, space1, alpha1};
use nom::combinator::map;
use nom::error::Error;
use nom::sequence::tuple;
use nom::{combinator::map_res, IResult};
use nom::bytes::complete::{take_till1, take_till};
use nom::multi::count;

#[derive(Debug)]
enum Op {
    Toggle,
    Turn(bool),
}

#[derive(Debug)]
struct ParseOpError(String);

impl FromStr for Op {
    type Err = ParseOpError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "toggle" => Ok(Op::Toggle),
            "turn on" => Ok(Op::Turn(true)),
            "turn off" => Ok(Op::Turn(true)),
            _ => Err(ParseOpError(std::format!("invalid token: {}", s))),
        }
    }
}

#[derive(Debug)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct ParseCoordsError(String);

impl FromStr for Coords {
    type Err = ParseCoordsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(",").collect();
        if parts.len() != 2 {
            return Err(ParseCoordsError("expected two parsts".to_string()));
        }
        let x = parts[0]
            .parse()
            .map_err(|_e| ParseCoordsError("error parsing x coordinate".to_string()))?;
        let y = parts[0]
            .parse()
            .map_err(|_e| ParseCoordsError("error parsing y coordinate".to_string()))?;

        Ok(Coords { x: x, y: y })
    }
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    map_res(
        alt((tag("toggle"), tag("turn on"), tag("turn off"))),
        |s: &str| s.parse(),
    )(input)


    // map_res(alt((take_word, take_words::<2>)), |s| s.parse())(input)
    // map_res(alt((take_words(1), take_words(2))), |s| s.parse())(input)
}

use nom::sequence::preceded;
use nom::combinator::recognize;
use nom::character::complete::space0;

fn take_word(input: &str) -> IResult<&str, &str> {
    preceded(space0, recognize(take_till1(|c| c == ' ')))(input)
}

// fn take_words<'i>(_n: usize) -> impl Parser<&'i str, &'i str, ()> {
//     move |input: &'i str| {
//         take_till1(|c| c == ' ')(input)
//     }
// }

// fn join_array_with_whitespace<const N: usize>(arr: [&str; N]) -> &str {
//     let joined_string: String = arr.iter().cloned().collect::<Vec<&str>>().join(" ");
//     &joined_string // cannot return reference to local variable `joined_string`
// }

fn take_words<const N: usize>(input: &str) -> IResult<&str, [&str; N]> {
    map_res(
        count(take_word, N),
        |words| words.try_into()
    )(input)
}

// fn take_words<const N: usize>(input: &str) -> IResult<&str, &str> {
//     map_res(
//         count(take_word, N),
//         |words| {
//             words.try_into().map(join_array_with_whitespace::<N>)
//             // let r = words.try_into();
//             // let s = r.map(join_array_with_whitespace);
//             // s
//         }
//     )(input)
// }


fn parse_coords(input: &str) -> IResult<&str, Coords> {
    // map_res(take_till1(|c| c == ' '), |s: &str| s.parse())(input)
    // map(take_word, |s| s.parse()) // this doesn't work, I think because of Err mismatches
    map_res(take_word, |s| s.parse())(input)
}

fn parse_nothing(_input: &str) -> IResult<&str, ()> {
    Ok(("", ()))
}

fn main() {
    println!("hello, nom!");

    let x = parse_nothing("hello world");
    println!("x = {:?}", x);

    let op: Op = "toggle".parse().unwrap();
    println!("op = {:?}", op);

    let op = parse_op("turn off");
    println!("op = {:?}", op);

    let coords = "100,101".parse::<Coords>();
    println!("coords = {:?}", coords);

    let result =
        tuple((parse_op, nom::character::complete::multispace0, parse_op))("turn on    toggle");
    println!("result = {:?}", result);

    let result = tuple((parse_op, char(' '), parse_coords, tag(" through "), parse_coords))(
        "turn on 100,200 through 180,220",
    );
    println!("result = {:?}", result);


    let result = take_word("hello world foo bar");
    println!("RESULT = {:?}", result);

    let result = take_word("hello");
    println!("RESULT = {:?}", result);

    let result = take_word(" foo");
    println!("RESULT = {:?}", result);

    let result = take_words::<2>("hello world");
    println!("RESULT = {:?}", result);
}

// TODO: parser ensures that rectangle corners are bottom left and top right
