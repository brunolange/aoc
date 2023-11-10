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
        let char_count_map: HashMap<char, usize> =
            self.name
                .chars()
                .filter(|c| *c != '-')
                .fold(HashMap::new(), |mut map, c| {
                    *map.entry(c).or_insert(0) += 1;
                    map
                });

        let count_char_map: HashMap<usize, Vec<char>> =
            char_count_map
                .into_iter()
                .fold(HashMap::new(), |mut map, (c, count)| {
                    map.entry(count).or_default().push(c);
                    map
                });

        // TODO: a most_common iterator for strings that yields sorted characters
        // self.name.most_common().take(5)
        let mut top =
            BinaryHeap::from_iter(count_char_map.into_iter().map(|(count, c)| (count, c)));

        let mut checksum = Vec::new();
        while checksum.len() < 5 {
            let (_, mut v) = top.pop().unwrap();
            v.sort();
            checksum.append(&mut v);
        }

        checksum
            .into_iter()
            .take(self.checksum.len())
            .collect::<String>()
            == self.checksum
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
                .map_while(Result::ok)
                .filter(|line| !line.starts_with("--")),
        ),
        Some(path) => {
            let file = File::open(path).expect("error reading file");
            Box::new(io::BufReader::new(file).lines().map_while(Result::ok))
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
