use std::{io::BufRead, str::FromStr};

type Registers = [u64; 2];
type Register = u64;

type Offset = i64;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
enum Instruction {
    HLF(Register),
    TPL(Register),
    INC(Register),
    JMP(Offset),
    JIE(Register, Offset),
    JIO(Register, Offset),
}

#[derive(Debug)]
struct InstructionParsingError(String);

impl FromStr for Instruction {
    type Err = InstructionParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split(' ').collect::<Vec<&str>>();
        let instruction = match words[0] {
            "hlf" => {
                let register = words[1].chars().next().unwrap() as Register - 'a' as Register;
                Instruction::HLF(register)
            }
            "tpl" => {
                let register = words[1].chars().next().unwrap() as Register - 'a' as Register;
                Instruction::TPL(register)
            }
            "inc" => {
                let register = words[1].chars().next().unwrap() as Register - 'a' as Register;
                Instruction::INC(register)
            }
            "jmp" => {
                let offset = words[1].parse::<Offset>().unwrap();
                Instruction::JMP(offset)
            }
            "jie" => {
                let register = words[1].chars().next().unwrap() as Register - 'a' as Register;
                let offset = words[2].parse::<Offset>().unwrap();
                Instruction::JIE(register, offset)
            }
            "jio" => {
                let register = words[1].chars().next().unwrap() as Register - 'a' as Register;
                let offset = words[2].parse::<Offset>().unwrap();
                Instruction::JIO(register, offset)
            }
            _ => {
                return Err(InstructionParsingError("invalid instruction".to_string()));
            }
        };
        Ok(instruction)
    }
}

type Program = Vec<Instruction>;

fn run(program: Program) -> Registers {
    let mut registers: Registers = [1, 0];
    let mut cursor: Offset = 0;
    loop {
        println!("cursor = {cursor}");
        if cursor < 0 || cursor as usize >= program.len() {
            break;
        }
        let instruction = &program[cursor as usize];
        println!("Evaluating {:?}", instruction);
        cursor = match instruction {
            Instruction::HLF(register) => {
                registers[*register as usize] /= 2;
                cursor + 1
            }
            Instruction::TPL(register) => {
                registers[*register as usize] *= 3;
                cursor + 1
            }
            Instruction::INC(register) => {
                registers[*register as usize] += 1;
                cursor + 1
            }
            Instruction::JMP(offset) => {
                let index = cursor as Offset + offset;
                if index < 0 {
                    break;
                } else {
                    index
                }
            }
            Instruction::JIE(register, offset) => {
                let offset = if registers[*register as usize] % 2 == 0 {
                    *offset
                } else {
                    1
                };
                cursor + offset
            }
            Instruction::JIO(register, offset) => {
                let offset = if registers[*register as usize] == 1 {
                    *offset
                } else {
                    1
                };
                cursor + offset
            }
        }
    }

    registers
}

fn main() {
    // let program = vec![
    //     Instruction::INC(0),
    //     Instruction::JIO(0, 2),
    //     Instruction::TPL(0),
    //     Instruction::INC(0),
    // ];
    // let registers = run(program);
    // println!("registers: {:?}", registers);

    let program: Program = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<Instruction>().unwrap())
        .collect();

    let registers = run(program);
    println!("registers: {:?}", registers);
}
