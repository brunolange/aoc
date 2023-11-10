use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseRoomError(String);

#[derive(Debug)]
struct Room {
    name: String,
    sector_id: usize,
    checksum: String,
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

fn main() {
    println!("Hello, world!");

    let room: Room = "aaaaa-bbb-z-y-x-123[abxyz]".parse().unwrap();
    println!("{:?}", room);

    let room: Room = "a-b-c-d-e-f-g-h-987[abcde]".parse().unwrap();
    println!("{:?}", room);

    let room: Room = "not-a-real-room-404[oarel]".parse().unwrap();
    println!("{:?}", room);

    let room: Room = "totally-real-room-200[decoy]".parse().unwrap();
    println!("{:?}", room);
}
