use std::str::FromStr;
use std::{fs::File, io::BufRead};

pub fn lines() -> Box<dyn Iterator<Item = String>> {
    match std::env::args().nth(1) {
        None => Box::new(std::io::stdin().lock().lines().filter_map(Result::ok)),
        Some(path) => {
            let file = File::open(path).expect("Error reading file");
            Box::new(std::io::BufReader::new(file).lines().filter_map(Result::ok))
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction = match s {
            "U" => Instruction::Up,
            "L" => Instruction::Left,
            "R" => Instruction::Right,
            "D" => Instruction::Down,
            _ => return Err(()),
        };
        Ok(instruction)
    }
}

#[derive(Clone, Copy, Debug)]
enum Button {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
}

fn step(button: Button, instruction: Instruction) -> Button {
    match button {
        Button::One => match instruction {
            Instruction::Down => Button::Three,
            _ => Button::One,
        },
        Button::Two => match instruction {
            Instruction::Right => Button::Three,
            Instruction::Down => Button::Six,
            _ => Button::Two,
        },
        Button::Three => match instruction {
            Instruction::Up => Button::One,
            Instruction::Left => Button::Two,
            Instruction::Down => Button::Seven,
            Instruction::Right => Button::Four,
        },
        Button::Four => match instruction {
            Instruction::Left => Button::Three,
            Instruction::Down => Button::Eight,
            _ => Button::Four,
        },
        Button::Five => match instruction {
            Instruction::Right => Button::Six,
            _ => Button::Five,
        },
        Button::Six => match instruction {
            Instruction::Up => Button::Two,
            Instruction::Left => Button::Five,
            Instruction::Down => Button::A,
            Instruction::Right => Button::Seven,
        },
        Button::Seven => match instruction {
            Instruction::Up => Button::Three,
            Instruction::Left => Button::Six,
            Instruction::Down => Button::B,
            Instruction::Right => Button::Eight,
        },
        Button::Eight => match instruction {
            Instruction::Up => Button::Four,
            Instruction::Left => Button::Seven,
            Instruction::Down => Button::C,
            Instruction::Right => Button::Nine,
        },
        Button::Nine => match instruction {
            Instruction::Left => Button::Eight,
            _ => Button::Nine,
        },
        Button::A => match instruction {
            Instruction::Up => Button::Six,
            Instruction::Right => Button::B,
            _ => Button::A,
        },
        Button::B => match instruction {
            Instruction::Up => Button::Seven,
            Instruction::Left => Button::A,
            Instruction::Down => Button::D,
            Instruction::Right => Button::C,
        },
        Button::C => match instruction {
            Instruction::Up => Button::Eight,
            Instruction::Left => Button::B,
            _ => Button::C,
        },
        Button::D => match instruction {
            Instruction::Up => Button::B,
            _ => Button::D,
        },
    }
}

fn decode(
    starting_button: Button,
    lines: impl Iterator<Item = String>,
) -> impl Iterator<Item = Button> {
    lines.scan(starting_button, |state, line| {
        let partial = line
            .chars()
            .map(|c| c.to_string().parse::<Instruction>().unwrap())
            .fold(state.clone(), step);
        *state = partial;
        Some(partial)
    })
}

fn main() {
    decode(Button::Five, lines()).for_each(|button| println!("{:?}", button));
}
