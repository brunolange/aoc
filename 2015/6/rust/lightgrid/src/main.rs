use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::bytes::complete::take_till1;
use nom::character::complete::char;
use nom::multi::count;
use nom::sequence::pair;
use nom::sequence::tuple;
use nom::{combinator::map_res, IResult};

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
    // This one makes npm intrude in the specifics of Op. I'm not a fan...
    // map_res(
    //     alt((tag("toggle"), tag("turn on"), tag("turn off"))),
    //     |s: &str| s.parse(),
    // )(input)

    // This doesn't work because we actually if we can take 2 words, we could always have take just 1!
    // alt tries the first, take_word, which does yield a word and it just returns that.
    // that word ("turn") happens to not be parseable into an Op, so we get an error here...
    // map_res(
    //     alt((
    //         take_word,
    //         recognize(take_words::<2>)
    //     )),
    //     |s| s.parse()
    // )(input)

    let parser = |s: &str| s.parse::<Op>();

    let x1 = map_res(take_word, parser)(input);

    match x1 {
        Ok(_) => x1,
        Err(_) => map_res(recognize(take_words::<2>), parser)(input),
    }
}

use nom::character::complete::space0;
use nom::combinator::recognize;
use nom::sequence::preceded;

fn take_word(input: &str) -> IResult<&str, &str> {
    preceded(space0, recognize(take_till1(|c| c == ' ')))(input)
}

fn take_words<const N: usize>(input: &str) -> IResult<&str, [&str; N]> {
    map_res(count(take_word, N), |words| words.try_into())(input)
}

// fn join_array_with_whitespace<const N: usize>(arr: [&str; N]) -> &str {
//     let joined_string: String = arr.iter().cloned().collect::<Vec<&str>>().join(" ");
//     &joined_string // cannot return reference to local variable `joined_string`
// }

// This doesn't work! Some hairy meddling with string slices!
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
    // map(take_word, |s| s.parse())(input) // this doesn't work, I think because of Err mismatches
    map_res(take_word, |s| s.parse())(input)
}

fn main() {
    println!("hello, nom!");

    let op: Op = "toggle".parse().unwrap();
    println!("op = {:?}", op);

    let op = parse_op("turn off");
    println!("op = {:?}", op);

    let op = recognize(take_words::<2>)("turn off");
    println!("opX = {:?}", op);

    let coords = "100,101".parse::<Coords>();
    println!("coords = {:?}", coords);

    let result =
        tuple((parse_op, nom::character::complete::multispace0, parse_op))("turn on    toggle");
    println!("result = {:?}", result);

    let result = tuple((
        parse_op,
        char(' '),
        parse_coords,
        tag(" through "),
        parse_coords,
    ))("turn on 100,200 through 180,220");
    println!("result = {:?}", result);

    let result = take_word("hello world foo bar");
    println!("RESULT = {:?}", result);

    let result = take_word("hello");
    println!("RESULT = {:?}", result);

    let result = take_word(" foo");
    println!("RESULT = {:?}", result);

    let result = take_words::<2>("hello world");
    println!("RESULT = {:?}", result);

    let r = pair(take_word, take_word)("foo bar");
    println!("r = {:?}", r);

    let r = recognize(pair(take_word, take_word))("foo bar baz");
    println!("r = {:?}", r);

    let r = recognize(take_words::<2>)("foo bar baz");
    println!("r = {:?}", r);
}

// TODO: parser ensures that rectangle corners are bottom left and top right
