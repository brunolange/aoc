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
    let table = vec![
        "Alice would gain 54 happiness units by sitting next to Bob.",
        "Alice would lose 79 happiness units by sitting next to Carol.",
        "Alice would lose 2 happiness units by sitting next to David.",
        "Bob would gain 83 happiness units by sitting next to Alice.",
        "Bob would lose 7 happiness units by sitting next to Carol.",
        "Bob would lose 63 happiness units by sitting next to David.",
        "Carol would lose 62 happiness units by sitting next to Alice.",
        "Carol would gain 60 happiness units by sitting next to Bob.",
        "Carol would gain 55 happiness units by sitting next to David.",
        "David would gain 46 happiness units by sitting next to Alice.",
        "David would lose 7 happiness units by sitting next to Bob.",
        "David would gain 41 happiness units by sitting next to Carol.",
    ]
    .into_iter()
    .map(|s| parse_pairing(s).unwrap())
    .map(|(_, pairing)| pairing)
    .fold(HashMap::new(), |mut acc, curr| {
        acc.entry(curr.first).or_insert(HashMap::new());
        acc.get_mut(curr.first)
            .unwrap()
            .insert(curr.second, curr.gain);
        acc
    });

    println!("table = {:?}", table);
    println!("# of guests = {}", table.len());
    let guests: Vec<&str> = table.keys().cloned().collect();
    let n = guests.len();
    println!("guests = {:?}", guests);

    // circular permutation!
    let result = guests
        .into_iter()
        .permutations(n)
        .take((1..=n - 1).product())
        .map(|gs| {
            let mut round = gs.clone();
            round.push(gs[0]);
            (
                gs,
                round
                    .clone()
                    .into_iter()
                    .zip(round.into_iter().skip(1))
                    .map(|(l, r)| {
                        let lr = table.get(l).unwrap().get(r).unwrap();
                        // println!("{} and {}: {}", l, r, lr);
                        let rl = table.get(r).unwrap().get(l).unwrap();
                        // println!("{} and {}: {}", r, l, rl);
                        lr + rl
                    })
                    .sum::<i32>(),
            )
        })
        .max_by_key(|(_, tally)| *tally)
        .unwrap();

    println!("result = {:?}", result);
}
