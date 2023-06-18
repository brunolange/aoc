use std::sync::Arc;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map_res;
use nom::{
    character::complete::{alpha1, digit1, space1},
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Clone, Debug)]
struct Reindeer {
    name: Arc<str>,
    fly_speed: u8,
    fly_duration: u8,
    rest_time: u16,
}

impl Reindeer {
    fn position_at(&self, t: usize) -> usize {
        t + 101
    }
}

fn parse_line(input: &str) -> IResult<&str, Reindeer> {
    let (input, name) = terminated(alpha1, space1)(input)?;
    let (input, fly_speed) = map_res(
        preceded(
            terminated(tag("can fly"), space1),
            terminated(digit1, preceded(space1, terminated(tag("km/s"), space1))),
        ),
        |v: &str| v.parse::<u8>(),
    )(input)?;

    let (input, fly_duration) = map_res(
        preceded(
            preceded(space0, terminated(tag("for"), space1)),
            terminated(
                digit1,
                preceded(space1, terminated(tag("seconds"), alt((space1, tag(","))))),
            ),
        ),
        |v: &str| v.parse::<u8>(),
    )(input)?;

    let (input, rest_time) = map_res(
        preceded(
            preceded(space0, terminated(tag("but then must rest for"), space1)),
            terminated(
                digit1,
                preceded(
                    space1,
                    terminated(tag("seconds"), preceded(space0, tag("."))),
                ),
            ),
        ),
        |v: &str| v.parse::<u16>(),
    )(input)?;

    Ok((
        input,
        Reindeer {
            name: name.into(),
            fly_speed,
            fly_duration,
            rest_time,
        },
    ))
}

fn main() {
    let lines = vec![
        "Vixen can fly 8 km/s for 8 seconds, but then must rest for 53 seconds.",
        "Blitzen can fly 13 km/s for 4 seconds, but then must rest for 49 seconds.",
        "Rudolph can fly 20 km/s for 7 seconds, but then must rest for 132 seconds.",
        "Cupid can fly 12 km/s for 4 seconds, but then must rest for 43 seconds.",
        "Donner can fly 9 km/s for 5 seconds, but then must rest for 38 seconds.",
        "Dasher can fly 10 km/s for 4 seconds, but then must rest for 37 seconds.",
        "Comet can fly 3 km/s for 37 seconds, but then must rest for 76 seconds.",
        "Prancer can fly 9 km/s for 12 seconds, but then must rest for 97 seconds.",
        "Dancer can fly 37 km/s for 1 seconds, but then must rest for 36 seconds.",
    ];
    let reindeers = lines.into_iter().map(|line| {
        let (_, reindeer) = parse_line(line).unwrap();
        reindeer
    });

    let winner = reindeers
        .map(|reindeer| (reindeer.position_at(2053), reindeer))
        .max_by_key(|(distance, _)| *distance)
        .unwrap();

    let (distance, reindeer) = winner;
    println!(
        "{} is the winner and has travelled {} kilometers.",
        reindeer.name, distance
    );
}
