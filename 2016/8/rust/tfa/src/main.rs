use std::{io::BufRead, str::FromStr};

enum RC {
    Row,
    Column,
}

struct Rectangle {
    width: usize,
    height: usize,
}

struct ColumnRotation {
    column: usize,
    by: usize,
}

struct RowRotation {
    row: usize,
    by: usize,
}

enum Instruction {
    Rect(Rectangle),
    RotateColumn(ColumnRotation),
    RotateRow(RowRotation),
}

#[derive(Debug)]
struct InstructionParseError(String);

impl FromStr for Instruction {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Ok(Instruction::Rotate(RC::Row, 0, 1))
        Ok(Instruction::Rect(Rectangle {
            width: 3,
            height: 2,
        }))
    }
}

#[derive(Debug)]
struct Grid<const R: usize, const C: usize>([[bool; C]; R]);

impl<const R: usize, const C: usize> std::fmt::Display for Grid<R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let table: String = self
            .0
            .map(|row| {
                let xs: String = row.map(|c| if c { '#' } else { '.' }).into_iter().collect();
                xs
            })
            .join("\n");

        write!(f, "{}", table)
    }
}

impl<const R: usize, const C: usize> Grid<R, C> {
    fn apply(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Rect(Rectangle { width, height }) => {
                for i in 0..height {
                    for j in 0..width {
                        self.0[i][j] = true;
                    }
                }
            }
            Instruction::RotateRow(RowRotation { row, by }) => self.0[row].rotate_right(by),
            Instruction::RotateColumn(ColumnRotation { column, by }) => {
                let mut xs: [bool; R] = self
                    .0
                    .iter()
                    .map(|row| row[column])
                    .collect::<Vec<bool>>()
                    .try_into()
                    .unwrap();

                xs.rotate_right(by);

                (0..R).for_each(|r| self.0[r][column] = xs[r]);
            }
        }
    }
}

fn main() {
    let mut grid = Grid([[false; 7]; 3]);

    // for line in std::io::stdin().lock().lines().map_while(Result::ok) {
    //     let instruction: Instruction = line.parse().expect("invalid instruction");
    //     grid.apply(instruction);
    // }
    grid.apply(Instruction::Rect(Rectangle {
        width: 3,
        height: 2,
    }));
    println!();
    println!("{}", grid);

    // grid.apply(Instruction::RotateRow(RowRotation { row: 0, by: 1 }));
    // println!();
    // println!("{}", grid);

    // grid.apply(Instruction::RotateRow(RowRotation { row: 0, by: 1 }));
    // println!();
    // println!("{}", grid);

    // grid.apply(Instruction::RotateRow(RowRotation { row: 0, by: 4 }));
    // println!();
    // println!("{}", grid);

    grid.apply(Instruction::RotateColumn(ColumnRotation {
        column: 0,
        by: 1,
    }));
    println!();
    println!("{}", grid);

    grid.apply(Instruction::RotateColumn(ColumnRotation {
        column: 4,
        by: 1,
    }));
    println!();
    println!("{}", grid);

    let count = grid
        .0
        .into_iter()
        .flat_map(|row| row.into_iter().filter(|c| *c))
        .count();

    println!("{count}");
}
