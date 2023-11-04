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
}

fn step(button: Button, instruction: Instruction) -> Button {
    match button {
        Button::One => match instruction {
            Instruction::Right => Button::Two,
            Instruction::Down => Button::Four,
            _ => Button::One,
        },
        Button::Two => match instruction {
            Instruction::Left => Button::One,
            Instruction::Right => Button::Three,
            Instruction::Down => Button::Five,
            _ => Button::Two,
        },
        Button::Three => match instruction {
            Instruction::Left => Button::Two,
            Instruction::Down => Button::Six,
            _ => Button::Three,
        },
        Button::Four => match instruction {
            Instruction::Up => Button::One,
            Instruction::Right => Button::Five,
            Instruction::Down => Button::Seven,
            _ => Button::Four,
        },
        Button::Five => match instruction {
            Instruction::Up => Button::Two,
            Instruction::Right => Button::Six,
            Instruction::Down => Button::Eight,
            Instruction::Left => Button::Four,
        },
        Button::Six => match instruction {
            Instruction::Up => Button::Three,
            Instruction::Down => Button::Nine,
            Instruction::Left => Button::Five,
            _ => Button::Six,
        },
        Button::Seven => match instruction {
            Instruction::Up => Button::Four,
            Instruction::Right => Button::Eight,
            _ => Button::Seven,
        },
        Button::Eight => match instruction {
            Instruction::Up => Button::Five,
            Instruction::Right => Button::Nine,
            Instruction::Left => Button::Seven,
            Instruction::Down => Button::Eight,
        },
        Button::Nine => match instruction {
            Instruction::Up => Button::Six,
            Instruction::Right => Button::Nine,
            Instruction::Left => Button::Eight,
            Instruction::Down => Button::Nine,
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
