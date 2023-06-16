use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::combinator::{map, map_res};
use nom::{
    character::complete::{alpha1, space0},
    sequence::{preceded, terminated},
    IResult,
};

use itertools::Itertools;

mod io;

#[derive(Debug)]
struct Pairing<'a> {
    first: &'a str,
    second: &'a str,
    gain: i32,
}

fn parse_pairing(input: &str) -> IResult<&str, Pairing> {
    let (input, first) = terminated(alpha1, space0)(input)?;
    let (input, mult) = preceded(
        terminated(tag("would"), space1),
        terminated(
            alt((map(tag("gain"), |_| 1), map(tag("lose"), |_| -1))),
            space1,
        ),
    )(input)?;

    let (input, value) = map_res(digit1, |v: &str| v.parse::<i32>())(input)?;
    let (input, second) = preceded(
        preceded(
            space0,
            terminated(tag("happiness units by sitting next to"), space1),
        ),
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
    let lines: Vec<String> = io::lines().collect();
    let table = lines
        .iter()
        .map(|line| {
            let (_, pairing) = parse_pairing(line).expect("Invalid pairing");
            pairing
        })
        .fold(HashMap::new(), |mut acc, curr| {
            acc.entry(curr.first).or_insert(HashMap::new());
            acc.get_mut(curr.first)
                .unwrap()
                .insert(curr.second, curr.gain);
            acc
        });

    let guests = table.keys();
    let n = guests.len();

    // circular permutation!
    let result = guests
        .into_iter()
        .permutations(n)
        .take((1..=n - 1).product())
        .map(|guests| {
            let mut round = guests.clone();
            round.push(guests[0]);

            let xs = round
                .clone()
                .into_iter()
                .zip(round.into_iter().skip(1))
                .map(|(l, r)| {
                    let lr = table.get(l).unwrap().get(r).unwrap();
                    let rl = table.get(r).unwrap().get(l).unwrap();
                    lr + rl
                });

            let total = xs.clone().sum::<i32>();
            let min = xs.min().unwrap();

            // the inclusion of the host will break the weakest link
            (guests, total - min)
        })
        .max_by_key(|(_, tally)| *tally)
        .unwrap();

    println!("result = {:?}", result);
}
