use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseRoomError(String);

#[derive(Debug)]
struct Room {
    name: String,
    sector_id: usize,
    checksum: String,
}

impl Room {
    fn is_real(&self) -> bool {
        // if self.checksum != self.checksum.chars().sorted().collect::<String>() {
        //     return false;
        // }
        let xs: HashMap<char, usize> =
            self.name
                .chars()
                .filter(|c| *c != '-')
                .fold(HashMap::new(), |mut map, c| {
                    *map.entry(c).or_insert(0) += 1;
                    map
                });

        let ys: HashMap<usize, Vec<char>> =
            xs.into_iter().fold(HashMap::new(), |mut map, (c, count)| {
                map.entry(count).or_insert(Vec::new()).push(c);
                map
            });

        let mut yys = BinaryHeap::from_iter(ys.into_iter().map(|(count, c)| (count, c)));

        let mut cs = Vec::new();
        while cs.len() < 5 {
            let (_, mut v) = yys.pop().unwrap();
            v.sort();
            cs.append(&mut v);
        }
        cs = cs[0..5].to_vec();
        cs.into_iter().collect::<String>() == self.checksum

        // TODO: a most_common iterator for strings that yields sorted characters
        // self.name.most_common().take(5)
    }
}

fn parse_name(s: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag("-"), alpha1)(s)
}

fn parse_usize(s: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(s)
}

fn parse_checksum(s: &str) -> IResult<&str, &str> {
    delimited(tag("["), alpha1, tag("]"))(s)
}

impl FromStr for Room {
    type Err = ParseRoomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, room_name_parts) =
            parse_name(s).map_err(|_| ParseRoomError("invalid room name".to_string()))?;

        let (s, sector_id) = preceded(tag("-"), parse_usize)(s)
            .map_err(|_| ParseRoomError("invalid room sector id".to_string()))?;

        let (_, checksum) =
            parse_checksum(s).map_err(|_| ParseRoomError("invalid room checksum".to_string()))?;

        Ok(Room {
            name: room_name_parts.join("-"),
            sector_id,
            checksum: checksum.to_string(),
        })
    }
}

pub fn lines() -> Box<dyn Iterator<Item = String>> {
    match std::env::args().nth(1) {
        None => Box::new(
            io::stdin()
                .lock()
                .lines()
                .filter_map(Result::ok)
                .filter(|line| !line.starts_with("--")),
        ),
        Some(path) => {
            let file = File::open(path).expect("error reading file");
            Box::new(io::BufReader::new(file).lines().filter_map(Result::ok))
        }
    }
}

fn main() {
    let output = lines()
        .map(|line| line.parse::<Room>().unwrap())
        .filter(|room| room.is_real())
        .map(|room| room.sector_id)
        .sum::<usize>();

    println!("output = {}", output);
}
