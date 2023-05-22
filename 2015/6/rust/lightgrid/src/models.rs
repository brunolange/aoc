use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::all_consuming;
use nom::sequence::{separated_pair, tuple};

use crate::parsers::{parse_grid_point, parse_usize};

/// Stores x and y coordinates of a grid that extends only to the first quadrant.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GridPoint {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct ParseGridPointError(String);

impl FromStr for GridPoint {
    type Err = ParseGridPointError;

    /// Parses the input into a point in the first quadrant of a cartesian grid.
    ///
    /// # Example
    /// ```rust
    /// use lightgrid::models::GridPoint;
    ///
    /// assert_eq!("100,101".parse::<GridPoint>().unwrap(), GridPoint{x:100, y:101});
    /// assert_eq!("999999,0".parse::<GridPoint>().unwrap(), GridPoint{x:999999, y:0});
    /// assert!("1,2,3".parse::<GridPoint>().is_err());
    /// assert!("-1,1".parse::<GridPoint>().is_err());
    /// assert!("2,-10".parse::<GridPoint>().is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = all_consuming(separated_pair(parse_usize, char(','), parse_usize));

        let (_, (x, y)) = parser(s)
            .map_err(|_| ParseGridPointError("Unable to parse coordinates".to_string()))?;

        Ok(GridPoint { x, y })
    }
}

#[derive(Debug, PartialEq)]
pub struct Rect {
    pub bottom_left_corner: GridPoint,
    pub top_right_corner: GridPoint,
}

impl Rect {
    pub fn new(p: &GridPoint, q: &GridPoint) -> Self {
        let (x0, y0) = (p.x, p.y);
        let (x1, y1) = (q.x, q.y);

        Rect {
            bottom_left_corner: GridPoint {
                x: x0.min(x1),
                y: y0.min(y1),
            },
            top_right_corner: GridPoint {
                x: x0.max(x1),
                y: y0.max(y1),
            },
        }
    }
}

pub struct RectIterator<'a> {
    pub rect: &'a Rect,
    pub current: GridPoint,
}

impl Rect {
    pub fn iter(&mut self) -> RectIterator {
        RectIterator {
            rect: self,
            current: self.bottom_left_corner,
        }
    }
}

impl<'a> Iterator for RectIterator<'a> {
    type Item = GridPoint;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = (self.current.x, self.current.y);
        let (max_x, max_y) = (self.rect.top_right_corner.x, self.rect.top_right_corner.y);

        if y > max_y {
            return None;
        }

        let grid_point = GridPoint {
            x: self.current.x,
            y: self.current.y,
        };

        let next_grid_point = if x >= max_x {
            GridPoint {
                x: self.rect.bottom_left_corner.x,
                y: y + 1,
            }
        } else {
            GridPoint { x: x + 1, y }
        };

        self.current = next_grid_point;

        Some(grid_point)
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Toggle(Rect),
    Turn(bool, Rect),
}

#[derive(Debug)]
pub struct ParseOpError(String);

impl FromStr for Op {
    type Err = ParseOpError;

    /// Parses this string slice into an Op (operation) instance.
    ///
    /// # Example
    /// ```rust
    /// use std::str::FromStr;
    /// use lightgrid::models::{GridPoint, Op, Rect};
    ///
    /// let op: Op = "turn on 0,0 through 1,1".parse().unwrap();
    /// assert_eq!(op,Op::Turn(true, Rect { bottom_left_corner: GridPoint { x: 0, y: 0 }, top_right_corner: GridPoint { x: 1, y: 1 } }));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (action, _, from, _, to)) = all_consuming(tuple((
            alt((tag("toggle"), tag("turn on"), tag("turn off"))),
            char(' '),
            parse_grid_point,
            tag(" through "),
            parse_grid_point,
        )))(s)
        .map_err(|_| ParseOpError("unable to parse line".to_string()))?;
        let rect = Rect::new(&from, &to);
        match action {
            "toggle" => Ok(Op::Toggle(rect)),
            "turn on" => Ok(Op::Turn(true, rect)),
            "turn off" => Ok(Op::Turn(false, rect)),
            _ => Err(ParseOpError(std::format!("invalid token: {}", s))),
        }
    }
}
