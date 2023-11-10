use nom::bytes::complete::tag;
use nom::sequence::preceded;
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;

use crate::parsers::{parse_checksum, parse_name, parse_usize};

#[derive(Debug)]
pub struct ParseRoomError(String);

#[derive(Debug)]
pub struct Room {
    pub name: String,
    pub sector_id: usize,
    pub checksum: String,
}

impl Room {
    pub fn is_real(&self) -> bool {
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
