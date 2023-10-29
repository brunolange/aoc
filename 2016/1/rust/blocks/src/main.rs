use std::collections::HashSet;
use std::str::FromStr;

const INPUT: &str = "R4, R4, L1, R3, L5, R2, R5, R1, L4, R3, L5, R2, L3, L4, L3, R1, R5, R1, L3, L1, R3, L1, R2, R2, L2, R5, L3, L4, R4, R4, R2, L4, L1, R5, L1, L4, R4, L1, R1, L2, R5, L2, L3, R2, R1, L194, R2, L4, R49, R1, R3, L5, L4, L1, R4, R2, R1, L5, R3, L5, L4, R4, R4, L2, L3, R78, L5, R4, R191, R4, R3, R1, L2, R1, R3, L1, R3, R4, R2, L2, R1, R4, L5, R2, L2, L4, L2, R1, R2, L3, R5, R2, L3, L3, R3, L1, L1, R5, L4, L4, L2, R5, R1, R4, L3, L5, L4, R5, L4, R5, R4, L3, L2, L5, R4, R3, L3, R1, L5, R5, R1, L3, R2, L5, R5, L3, R1, R4, L5, R4, R2, R3, L4, L5, R3, R4, L5, L5, R4, L4, L4, R1, R5, R3, L1, L4, L3, L4, R1, L5, L1, R2, R2, R4, R4, L5, R4, R1, L1, L1, L3, L5, L2, R4, L3, L5, L4, L1, R3";

#[derive(Debug)]
enum Turn {
    Right,
    Left,
}

impl FromStr for Turn {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s {
            "L" => Turn::Left,
            "R" => Turn::Right,
            _ => panic!("invalid directon"),
        };
        Ok(direction)
    }
}

#[derive(Debug)]
enum Orientation {
    East,
    North,
    West,
    South,
}

#[derive(Debug)]
struct Instruction {
    turn: Turn,
    length: usize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, rest) = s.split_at(1);
        let direction: Turn = head.parse()?;
        let length = rest.parse().unwrap();
        Ok(Instruction {
            turn: direction,
            length,
        })
    }
}

type Point = (i32, i32);

#[derive(Debug)]
struct Location {
    position: Point,
    orientation: Orientation,
}

impl Default for Location {
    fn default() -> Self {
        Location {
            position: (0, 0),
            orientation: Orientation::North,
        }
    }
}

fn step(location: &Location, instruction: &Instruction) -> Location {
    let displacement = instruction.length as i32;
    let (x, y) = location.position;
    match (&location.orientation, &instruction.turn) {
        (Orientation::East, Turn::Right) => Location {
            position: (x, y - displacement),
            orientation: Orientation::South,
        },
        (Orientation::East, Turn::Left) => Location {
            position: (x, y + displacement),
            orientation: Orientation::North,
        },

        (Orientation::North, Turn::Right) => Location {
            position: (x + displacement, y),
            orientation: Orientation::East,
        },
        (Orientation::North, Turn::Left) => Location {
            position: (x - displacement, y),
            orientation: Orientation::West,
        },

        (Orientation::West, Turn::Right) => Location {
            position: (x, y + displacement),
            orientation: Orientation::North,
        },
        (Orientation::West, Turn::Left) => Location {
            position: (x, y - displacement),
            orientation: Orientation::South,
        },

        (Orientation::South, Turn::Right) => Location {
            position: (x - displacement, y),
            orientation: Orientation::West,
        },
        (Orientation::South, Turn::Left) => Location {
            position: (x + displacement, y),
            orientation: Orientation::East,
        },
    }
}

fn main() {
    let final_position = INPUT
        .split(", ")
        .map(|s| s.parse::<Instruction>().unwrap())
        .fold(Location::default(), |acc, curr| step(&acc, &curr));

    let (x, y) = final_position.position;

    // manhattan distance
    let distance = x.abs() + y.abs();

    println!("{:?}", final_position);
    println!("distance = {}", distance);

    let mut point_set = HashSet::new();
    let mut target_point: Option<Point> = None;
    let mut location = Location::default();
    for instruction in INPUT.split(", ").map(|s| s.parse::<Instruction>().unwrap()) {
        location = step(&location, &instruction);
        let point = location.position;
        if point_set.contains(&point) {
            target_point = Some(point);
            break;
        }
        point_set.insert(point);
    }

    let point = target_point.unwrap();
    let (x, y) = point;

    println!("target_point = {:?}", point);
    println!("{}", x.abs() + y.abs())
}
