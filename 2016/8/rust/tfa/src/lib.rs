use std::fmt::Write;
use std::str::FromStr;

mod parsers;

use nom::branch::alt;
use parsers::{parse_column_rotation, parse_rect, parse_row_rotation};

#[derive(PartialEq, Eq, Debug)]
pub struct Rectangle {
    width: usize,
    height: usize,
}

#[derive(PartialEq, Eq, Debug)]
pub struct ColumnRotation {
    column: usize,
    by: usize,
}

#[derive(PartialEq, Eq, Debug)]
pub struct RowRotation {
    row: usize,
    by: usize,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Instruction {
    Rect(Rectangle),
    RotateColumn(ColumnRotation),
    RotateRow(RowRotation),
}

#[derive(Debug)]
pub struct InstructionParseError(String);

impl FromStr for Instruction {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = alt((parse_rect, parse_row_rotation, parse_column_rotation));

        let (_, instruction) = parser(s).map_err(|_| InstructionParseError("err".to_string()))?;
        Ok(instruction)
    }
}

#[derive(Debug)]
pub struct Grid<const C: usize, const R: usize>(pub [[bool; C]; R]);

impl<const C: usize, const R: usize> Grid<C, R> {
    pub fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Rect(Rectangle { width, height }) => {
                for i in 0..*height {
                    for j in 0..*width {
                        self.0[i][j] = true;
                    }
                }
            }
            Instruction::RotateRow(RowRotation { row, by }) => self.0[*row].rotate_right(*by),
            Instruction::RotateColumn(ColumnRotation { column, by }) => {
                let mut xs: [bool; R] = self
                    .0
                    .iter()
                    .map(|row| row[*column])
                    .collect::<Vec<bool>>()
                    .try_into()
                    .unwrap();

                xs.rotate_right(*by);

                (0..R).for_each(|r| self.0[r][*column] = xs[r]);
            }
        }
    }
}

impl<const C: usize, const R: usize> std::fmt::Display for Grid<C, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let table: String = self
            .0
            .map(|row| {
                row.into_iter()
                    .enumerate()
                    .fold(String::new(), |mut output, (i, cell)| {
                        let c = if cell { '#' } else { '.' };
                        let indent = if i != 0 && i % 5 == 0 {
                            " ".repeat(5)
                        } else {
                            "".to_string()
                        };
                        let _ = write!(output, "{indent}{c}");
                        output
                    })
            })
            .join("\n");

        write!(f, "{}", table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect() {
        let mut grid = Grid([[false; 3]; 2]);
        let instr = Instruction::Rect(Rectangle {
            width: 1,
            height: 1,
        });
        grid.apply(&instr);
        assert_eq!(grid.0, [[true, false, false], [false, false, false]]);

        grid.apply(&instr);
        assert_eq!(grid.0, [[true, false, false], [false, false, false]]);

        grid.apply(&Instruction::Rect(Rectangle {
            width: 3,
            height: 2,
        }));
        assert_eq!(grid.0, [[true, true, true], [true, true, true]]);
    }

    #[test]
    fn test_rotate_row() {
        let mut grid = Grid([[false; 3]; 2]);
        grid.apply(&Instruction::Rect(Rectangle {
            width: 1,
            height: 1,
        }));

        let instr = Instruction::RotateRow(RowRotation { row: 0, by: 1 });
        grid.apply(&instr);
        assert_eq!(grid.0, [[false, true, false], [false, false, false]]);

        grid.apply(&instr);
        assert_eq!(grid.0, [[false, false, true], [false, false, false]]);
    }

    #[test]
    fn test_rotate_column() {
        let mut grid = Grid([[false; 3]; 2]);
        grid.apply(&Instruction::Rect(Rectangle {
            width: 1,
            height: 1,
        }));

        let instr = Instruction::RotateColumn(ColumnRotation { column: 0, by: 1 });
        grid.apply(&instr);
        assert_eq!(grid.0, [[false, false, false], [true, false, false]]);

        grid.apply(&instr);
        assert_eq!(grid.0, [[true, false, false], [false, false, false]]);
    }

    #[test]
    fn test_rect_instruction_parser() {
        assert_eq!(
            "rect 123x321".parse::<Instruction>().unwrap(),
            Instruction::Rect(Rectangle {
                width: 123,
                height: 321
            })
        );
    }

    #[test]
    fn test_rotate_column_instruction_parser() {
        assert_eq!(
            "rotate column x=32 by 100".parse::<Instruction>().unwrap(),
            Instruction::RotateColumn(ColumnRotation {
                column: 32,
                by: 100
            })
        );
    }

    #[test]
    fn test_rotate_row_instruction_parser() {
        assert_eq!(
            "rotate row y=32 by 100".parse::<Instruction>().unwrap(),
            Instruction::RotateRow(RowRotation { row: 32, by: 100 })
        );
    }
}
