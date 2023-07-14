use std::collections::{BinaryHeap, HashMap};
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
    pub fn distance_at(&self, t: usize) -> usize {
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

pub fn race_1(reindeers: &Vec<Reindeer>, t: usize) -> (&Reindeer, usize) {
    let winner = reindeers
        .iter()
        .map(|reindeer| (reindeer.distance_at(t), reindeer))
        .max_by_key(|(distance, _)| *distance)
        .unwrap();

    let (distance, reindeer) = winner;
    (reindeer, distance)
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
    type Item = [(usize, Vec<&'a Reindeer>); K];

    fn next(&mut self) -> Option<Self::Item> {
        if self.reindeers.len() < K {
            return None;
        }

        self.t += 1;

        let mut distance_map: HashMap<usize, Vec<&Reindeer>> = HashMap::new();
        for r in self.reindeers {
            let distance = r.distance_at(self.t);
            // println!("- {} @ {}", r.name, distance);
            distance_map
                .entry(distance)
                .and_modify(|rs| rs.push(r))
                .or_insert_with(|| vec![r]);
        }

        let mut heap = BinaryHeap::from_iter(distance_map.keys());
        let top_k = (0..K)
            .map(|_| heap.pop().unwrap())
            .map(|distance| {
                let d = *distance;
                let v = distance_map[distance].clone();
                (d, v)
            })
            .collect::<Vec<(usize, Vec<&Reindeer>)>>()
            .try_into()
            .unwrap();

        Some(top_k)
    }
}

pub fn race_2(reindeers: &Vec<Reindeer>, t: usize) -> (Vec<String>, usize) {
    let mut score_board: HashMap<&Reindeer, usize> =
        reindeers.into_iter().map(|r| (r, 0)).collect();

    let race = Race { reindeers };
    for (_i, [(_distance, first), (_, second), (_, third)]) in race.top::<3>().take(t).enumerate() {
        for f in first {
            // println!("\tt = {}: {} @ {}", i + 1, f.name, distance);
            score_board.entry(f).and_modify(|score| *score += 1);
        }
        for s in second {
            score_board.entry(s).and_modify(|score| *score += 0);
        }
        for t in third {
            score_board.entry(t).and_modify(|score| *score += 0);
        }
    }

    let max_score = *score_board.values().max().unwrap();
    let winners = score_board
        .iter()
        .filter(|(_, &score)| score == max_score)
        .map(|(r, _)| r.name.as_ref().to_owned())
        .collect::<Vec<String>>();

    (winners, max_score)
}
