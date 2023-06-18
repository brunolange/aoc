use std::collections::{BinaryHeap, HashMap, HashSet};
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

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Reindeer {
    pub name: Arc<str>,
    pub fly_speed: usize,
    pub fly_duration: usize,
    pub rest_time: usize,
}

impl Reindeer {
    pub fn position_at(&self, t: usize) -> usize {
        let period = self.fly_duration + self.rest_time;
        let (quotient, remainder) = (t / period, t % period);

        let total_rest_time = quotient * self.rest_time
            + std::cmp::max(remainder as i64 - self.fly_duration as i64, 0) as usize;

        self.fly_speed * (t - total_rest_time)
    }
}

pub fn parse_line(input: &str) -> IResult<&str, Reindeer> {
    let (input, name) = terminated(alpha1, space1)(input)?;
    let (input, fly_speed) = map_res(
        preceded(
            terminated(tag("can fly"), space1),
            terminated(digit1, preceded(space1, terminated(tag("km/s"), space1))),
        ),
        |v: &str| v.parse::<usize>(),
    )(input)?;

    let (input, fly_duration) = map_res(
        preceded(
            preceded(space0, terminated(tag("for"), space1)),
            terminated(
                digit1,
                preceded(space1, terminated(tag("seconds"), alt((space1, tag(","))))),
            ),
        ),
        |v: &str| v.parse::<usize>(),
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
        |v: &str| v.parse::<usize>(),
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

// fn f(reindeers: &Vec<Reindeer>, t: usize) {
//     let score_board = (1..=t).fold(
//         HashMap::from_iter(reindeers.clone().into_iter().map(|r| (r, 0))),
//         |mut sb: HashMap<Reindeer, usize>, _| {
//             for (_, score) in sb.iter_mut() {
//                 *score += 1;
//             }
//             sb
//         },
//     );

//     println!("{:?}", score_board)
//     // for i in 1..=t {
//     //     step(&reindeers, i)
//     // }
// }

pub fn race_1(reindeers: &Vec<Reindeer>, t: usize) -> Option<(&Reindeer, usize)> {
    let winner = reindeers
        .iter()
        .map(|reindeer| (reindeer.position_at(t), reindeer))
        .map(|(d, r)| {
            println!("{}: {}", r.name, d);
            (d, r)
        })
        .max_by_key(|(distance, _)| *distance)
        .unwrap();

    let (distance, reindeer) = winner;
    Some((reindeer, distance))
}

struct Race<'a> {
    reindeers: &'a Vec<Reindeer>,
}

impl<'a> Race<'a> {
    pub fn top<const K: usize>(&self) -> Top<K> {
        Top::<K> {
            reindeers: self.reindeers,
            t: 0,
        }
    }
}

struct Top<'a, const K: usize> {
    reindeers: &'a Vec<Reindeer>,
    t: usize,
}

impl<'a, const K: usize> Iterator for Top<'a, K> {
    type Item = [(usize, &'a str); K];

    fn next(&mut self) -> Option<Self::Item> {
        // update
        self.t += 1;

        let mut max_heap = BinaryHeap::from_iter(
            self.reindeers
                .into_iter()
                .map(|r| (r.position_at(self.t), r.name.as_ref())),
        );

        if max_heap.len() < K {
            return None;
        }

        let top_k = (0..K)
            .map(|_| max_heap.pop().unwrap())
            .collect::<Vec<(usize, &str)>>()
            .try_into()
            .unwrap();

        Some(top_k)
    }
}

pub fn race_2(reindeers: &Vec<Reindeer>, t: usize) -> (String, usize) {
    let mut score_board: HashMap<&str, usize> = reindeers
        .into_iter()
        .map(|r| (r.name.as_ref(), 0))
        .collect();

    let race = Race { reindeers };
    for [(_, first), (_, second), (_, third)] in race.top::<3>().take(t) {
        // println!("t = {}, winner = {} @ {}", i, first, p1);
        // i += 1;
        score_board.entry(first).and_modify(|score| *score += 1);
        score_board.entry(second).and_modify(|score| *score += 0);
        score_board.entry(third).and_modify(|score| *score += 0);
    }

    let (winner, score) = score_board
        .into_iter()
        .max_by_key(|(_, score)| *score)
        .unwrap();

    (winner.to_owned(), score)
}
