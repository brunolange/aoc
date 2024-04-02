use itertools::Itertools;
use std::convert::TryInto;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Safe,
    Trap,
}

impl FromStr for Tile {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Tile::Safe),
            "^" => Ok(Tile::Trap),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Safe => '.',
                Tile::Trap => '^',
            }
        )
    }
}

#[derive(Clone, Debug)]
struct Row(Vec<Tile>);

struct RowIter {
    first: bool,
    curr: Row,
}

impl Row {
    fn into_iter(self) -> RowIter {
        RowIter {
            first: true,
            curr: self,
        }
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|t| format!("{}", t)).join(""))
    }
}

fn next(r: &Row) -> Row {
    let mut xr = vec![];
    xr.push(Tile::Safe);
    xr.append(&mut r.0.clone());
    xr.push(Tile::Safe);

    Row(xr
        .into_iter()
        .tuple_windows()
        .map(|triplet| match triplet {
            (Tile::Trap, Tile::Trap, Tile::Safe) => Tile::Trap,
            (Tile::Safe, Tile::Trap, Tile::Trap) => Tile::Trap,
            (Tile::Trap, Tile::Safe, Tile::Safe) => Tile::Trap,
            (Tile::Safe, Tile::Safe, Tile::Trap) => Tile::Trap,
            _ => Tile::Safe,
        })
        .collect())
}

impl Iterator for RowIter {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.first {
            self.curr = next(&self.curr);
        }

        self.first = false;

        Some(self.curr.clone())
    }
}
fn main() {
    let [first_row, number_of_rows]: [String; 2] = std::env::args()
        .skip(1)
        .collect::<Vec<_>>()
        .try_into()
        .expect("invalid usage");

    let take = number_of_rows
        .parse::<usize>()
        .expect("invalid number of rows");

    let row = Row(first_row
        .chars()
        .map(|c| c.to_string().parse::<Tile>().unwrap())
        .collect());

    let count: usize = row
        .into_iter()
        .take(take)
        // .inspect(|r| println!("{}", r))
        .map(|r| r.0.into_iter().filter(|t| *t == Tile::Safe).count())
        .sum();

    println!("{count}");
}
