use std::{
    collections::LinkedList,
    fmt::{Display, Formatter},
    process,
    str::FromStr,
};

const OPEN: [char; 5] = ['b', 'c', 'd', 'e', 'f'];

fn is_open(c: char) -> bool {
    OPEN.contains(&c)
}

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => "U",
                Direction::Down => "D",
                Direction::Left => "L",
                Direction::Right => "R",
            }
        )
    }
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Node {
    position: (usize, usize),
    code: String,
    path: Vec<Direction>,
    depth: usize,
}

impl Node {
    fn neighbors(&self) -> Vec<Node> {
        format!("{:x}", md5::compute(&self.code))
            .chars()
            .zip([
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ])
            .filter(|(c, d)| {
                if !is_open(*c) {
                    return false;
                }
                let (row, column) = self.position;
                match d {
                    Direction::Up => row > 0,
                    Direction::Left => column > 0,
                    Direction::Down => row < 3,
                    Direction::Right => column < 3,
                }
            })
            .map(|(_, direction)| {
                let (row, column) = self.position;
                let mut path = self.path.clone();
                path.push(direction.clone());
                Node {
                    position: match direction {
                        Direction::Up => (row - 1, column),
                        Direction::Down => (row + 1, column),
                        Direction::Left => (row, column - 1),
                        Direction::Right => (row, column + 1),
                    },
                    code: format!(
                        "{}{}",
                        self.code,
                        match direction {
                            Direction::Up => 'U',
                            Direction::Down => 'D',
                            Direction::Left => 'L',
                            Direction::Right => 'R',
                        }
                    ),
                    path,
                    depth: self.depth + 1,
                }
            })
            .collect()
    }
}

fn shortest_path(passcode: &str) -> Option<Vec<Direction>> {
    let start = Node {
        position: (0, 0),
        code: passcode.to_string(),
        path: vec![],
        depth: 0,
    };

    let mut queue: LinkedList<Node> = LinkedList::new();
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        // println!("{}{:?}", "    ".repeat(node.depth), node);

        if node.position == (3, 3) {
            return Some(node.path.clone());
        }

        for neighbor in node.neighbors() {
            queue.push_back(neighbor);
        }
    }

    None
}

fn longest_path(passcode: &str) -> Option<Vec<Direction>> {
    let start = Node {
        position: (0, 0),
        code: passcode.to_string(),
        path: vec![],
        depth: 0,
    };

    let mut queue: LinkedList<Node> = LinkedList::new();
    queue.push_back(start);

    let mut longest_path: Option<Vec<Direction>> = None;
    while let Some(node) = queue.pop_front() {
        // println!("{}{:?}", "    ".repeat(node.depth), node);

        if node.position == (3, 3) {
            let len = node.path.len();
            let longest_len = longest_path.clone().map(|p| p.len()).unwrap_or(0);
            if len > longest_len {
                longest_path = Some(node.path.clone());
            }
        } else {
            for neighbor in node.neighbors() {
                queue.push_back(neighbor);
            }
        }
    }

    longest_path
}

fn fmt(directions: &Vec<Direction>) -> String {
    format!(
        "{}",
        directions
            .into_iter()
            .map(|d| format!("{}", d))
            .collect::<Vec<_>>()
            .join("")
    )
}

fn main() {
    if let Some(path) = shortest_path("ulqzkmiv") {
        println!("{}", fmt(&path));
    } else {
        eprintln!("ERROR: there is no path.");
        process::exit(1);
    }

    if let Some(path) = longest_path("ulqzkmiv") {
        println!("{}", path.len());
    } else {
        eprintln!("ERROR: there is no path.");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(shortest_path("hijkl") == None);
        assert!(
            shortest_path("ihgpwlah")
                == Some(vec![
                    Direction::Down,
                    Direction::Down,
                    Direction::Right,
                    Direction::Right,
                    Direction::Right,
                    Direction::Down,
                ])
        );

        assert!(
            shortest_path("kglvqrro")
                == Some(vec![
                    Direction::Down,
                    Direction::Down,
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Left,
                    Direction::Right,
                    Direction::Right,
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Down,
                ])
        );

        assert!(
            shortest_path("ulqzkmiv")
                == Some(vec![
                    Direction::Down,
                    Direction::Right,
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Right,
                    Direction::Up,
                    Direction::Down,
                    Direction::Down,
                    Direction::Left,
                    Direction::Left,
                    Direction::Down,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Left,
                    Direction::Right,
                    Direction::Left,
                    Direction::Down,
                    Direction::Up,
                    Direction::Up,
                    Direction::Down,
                    Direction::Down,
                    Direction::Down,
                    Direction::Right,
                    Direction::Right,
                ])
        );
    }
}
