use std::{collections::HashMap, io::BufRead, str::FromStr};

#[derive(Eq, PartialEq, Hash, Debug)]
struct Register(String);

#[derive(Debug)]
enum Copyable {
    Value(i32),
    Reg(Register),
}

impl FromStr for Copyable {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<i32>()
            .map_or(Copyable::Reg(Register(s.to_owned())), |v| {
                Copyable::Value(v)
            }))
    }
}

#[derive(Debug)]
enum Jumpable {
    Value(i32),
    Reg(Register),
}

#[derive(Debug)]
enum Instruction {
    Copy(Copyable, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Jumpable, i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(' ').collect();
        let instruction = match tokens[..] {
            ["cpy", x, y] => Instruction::Copy(
                x.parse().expect("invalid copy value"),
                Register(y.to_string()),
            ),
            ["jnz", x, y] => {
                let jumpable = if let Ok(x) = x.parse::<i32>() {
                    Jumpable::Value(x)
                } else {
                    Jumpable::Reg(Register(x.to_string()))
                };
                Instruction::Jnz(jumpable, y.parse().expect("invalid jnz offset"))
            }
            ["inc", x] => Instruction::Inc(Register(x.to_string())),
            ["dec", x] => Instruction::Dec(Register(x.to_string())),
            _ => panic!("invalid instruction: {}", s),
        };
        Ok(instruction)
    }
}

fn main() {
    let instructions: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect();

    let mut cursor: i32 = 0;
    let mut state = HashMap::new();

    while cursor < instructions.len() as i32 {
        if cursor < 0 {
            panic!("underflow!");
        }

        let instruction = &instructions[cursor as usize];

        let jmp = match instruction {
            Instruction::Copy(copyable, register) => {
                let value = match copyable {
                    Copyable::Value(value) => *value,
                    Copyable::Reg(from) => *state.get(from).expect("missing register"),
                };
                state.insert(register, value);
                1
            }
            Instruction::Jnz(jumpable, offset) => {
                let value = match jumpable {
                    Jumpable::Value(value) => *value,
                    Jumpable::Reg(register) => *state.entry(register).or_insert(0),
                };
                if value != 0 {
                    *offset
                } else {
                    1
                }
            }
            Instruction::Inc(register) => {
                state
                    .entry(register)
                    .and_modify(|value| *value += 1)
                    .or_insert(1);
                1
            }
            Instruction::Dec(register) => {
                state
                    .entry(register)
                    .and_modify(|value| *value -= 1)
                    .or_insert(-1);
                1
            }
        };
        cursor += jmp;
    }

    println!("state = {:?}", state);
}
