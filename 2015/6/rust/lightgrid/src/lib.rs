use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_till1;
use nom::character::complete::{char, digit1, space0};
use nom::combinator::{all_consuming, map_res, recognize};
use nom::multi::count;
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
pub struct Coords(usize, usize);

#[derive(Debug)]
pub struct ParseCoordsError(String);

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

impl FromStr for Coords {
    type Err = ParseCoordsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = all_consuming(separated_pair(parse_usize, char(','), parse_usize));

        let (_, (x, y)) = parser(s).map_err(|_| ParseCoordsError("Unable to parse coordinates".to_string()))?;

        Ok(Coords(x, y))
    }
}

#[derive(Debug, PartialEq)]
pub struct Rect {
    pub bottom_left_corner: Coords,
    pub top_right_corner: Coords,
}

impl Rect {
    pub fn new(p: Coords, q: Coords) -> Self {
        let (x0, y0) = (p.0, p.1);
        let (x1, y1) = (q.0, q.1);

        Rect {
            bottom_left_corner: Coords(x0.min(x1), y0.min(y1)),
            top_right_corner: Coords(x0.max(x1), y0.max(y1)),
        }
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
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (action, _, from, _, to)) = all_consuming(tuple((
            alt((tag("toggle"), tag("turn on"), tag("turn off"))),
            char(' '),
            parse_coords,
            tag(" through "),
            parse_coords,
        )))(s)
        .map_err(|_| ParseOpError("unable to parse line".to_string()))?;
        let rect = Rect::new(from, to);
        match action {
            "toggle" => Ok(Op::Toggle(rect)),
            "turn on" => Ok(Op::Turn(true, rect)),
            "turn off" => Ok(Op::Turn(false, rect)),
            _ => return Err(ParseOpError(std::format!("invalid token: {}", s))),
        }
    }
}

pub fn parse_coords(input: &str) -> IResult<&str, Coords> {
    // map(take_word, |s| s.parse())(input) // this doesn't work, I think because of Err mismatches
    map_res(take_word, |s| s.parse())(input)
}

pub fn take_word(input: &str) -> IResult<&str, &str> {
    preceded(space0, recognize(take_till1(|c| c == ' ')))(input)
}

pub fn take_words<const N: usize>(input: &str) -> IResult<&str, [&str; N]> {
    map_res(count(take_word, N), |words| words.try_into())(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect() {
        assert_eq!(
            Rect::new(Coords(10, 10), Coords(11, 11)),
            Rect {
                bottom_left_corner: Coords(10, 10),
                top_right_corner: Coords(11, 11),
            }
        );
        assert_eq!(
            Rect::new(Coords(11, 11), Coords(10, 10)),
            Rect {
                bottom_left_corner: Coords(10, 10),
                top_right_corner: Coords(11, 11),
            }
        );

        assert_eq!(
            Rect::new(Coords(10, 10), Coords(11, 9)),
            Rect {
                bottom_left_corner: Coords(10, 9),
                top_right_corner: Coords(11, 10),
            }
        );
        assert_eq!(
            Rect::new(Coords(11, 9), Coords(10, 10)),
            Rect {
                bottom_left_corner: Coords(10, 9),
                top_right_corner: Coords(11, 10),
            }
        );
    }

    #[test]
    fn test_toggle_parse() {
        assert_eq!(
            "toggle 1,2 through 3,4".parse::<Op>().unwrap(),
            Op::Toggle(Rect {
                bottom_left_corner: Coords(1, 2),
                top_right_corner: Coords(3, 4),
            })
        );

        assert!("toggle 1,2 through 3,4 ".parse::<Op>().is_err());
        assert!(" toggle 1,2 through 3,4".parse::<Op>().is_err());
        assert!("toggle 1,2,3 through 4,5".parse::<Op>().is_err());
        assert!("toggle 1,2,3 through 4,5".parse::<Op>().is_err());
    }

    #[test]
    fn test_turn_on_parse() {
        assert_eq!(
            "turn on 1,2 through 3,4".parse::<Op>().unwrap(),
            Op::Turn(
                true,
                Rect {
                    bottom_left_corner: Coords(1, 2),
                    top_right_corner: Coords(3, 4),
                }
            )
        );

        assert!("turn on 1,2 through 3,4 ".parse::<Op>().is_err());
        assert!(" turn on 1,2 through 3,4".parse::<Op>().is_err());
        assert!("turn on 1,2,3 through 4,5".parse::<Op>().is_err());
        assert!("turn on 1,2,3 through 4,5".parse::<Op>().is_err());
    }

    #[test]
    fn test_turn_off_parse() {
        assert_eq!(
            "turn off 1,2 through 3,4".parse::<Op>().unwrap(),
            Op::Turn(
                false,
                Rect {
                    bottom_left_corner: Coords(1, 2),
                    top_right_corner: Coords(3, 4),
                }
            )
        );

        assert!("turn on 1,2 through 3,4 ".parse::<Op>().is_err());
        assert!(" turn on 1,2 through 3,4".parse::<Op>().is_err());
        assert!("turn on 1,2,3 through 4,5".parse::<Op>().is_err());
        assert!("turn on 1,2,3 through 4,5".parse::<Op>().is_err());
    }

    #[test]
    fn test_take_word() {
        assert_eq!(take_word("hello").unwrap(), ("", "hello"));
        assert_eq!(take_word("hello ").unwrap(), (" ", "hello"));
        assert_eq!(take_word(" hello").unwrap(), ("", "hello"));
        assert_eq!(take_word(" hello ").unwrap(), (" ", "hello"));
        assert_eq!(take_word("hello world").unwrap(), (" world", "hello"));
        assert_eq!(take_word(" hello world").unwrap(), (" world", "hello"));
        assert_eq!(take_word("  hello world").unwrap(), (" world", "hello"));
    }

    #[test]
    fn test_take_words() {
        assert_eq!(take_words::<1>("hello").unwrap(), ("", ["hello"]));
        assert_eq!(
            take_words::<1>("hello world").unwrap(),
            (" world", ["hello"])
        );
        assert_eq!(
            take_words::<2>("hello world").unwrap(),
            ("", ["hello", "world"])
        );
        assert_eq!(
            take_words::<2>("hello hello world").unwrap(),
            (" world", ["hello", "hello"])
        );
        assert_eq!(
            take_words::<3>("hello hello world").unwrap(),
            ("", ["hello", "hello", "world"])
        );
        assert_eq!(
            take_words::<3>("    hello hello world").unwrap(),
            ("", ["hello", "hello", "world"])
        );
        assert_eq!(
            take_words::<3>("    hello hello world ").unwrap(),
            (" ", ["hello", "hello", "world"])
        );
        assert!(take_words::<1>("").is_err());
        assert!(take_words::<1>("   ").is_err());
        assert!(take_words::<2>("hello").is_err());
        assert!(take_words::<2>("hello  ").is_err());
        assert!(take_words::<2>("   hello").is_err());
        assert!(take_words::<2>("   hello   ").is_err());
    }

    #[test]
    fn test_parse_coords() {
        assert_eq!("100,101".parse::<Coords>().unwrap(), Coords(100, 101));
        assert_eq!("999999,0".parse::<Coords>().unwrap(), Coords(999999, 0));
        assert!("1,2,3".parse::<Coords>().is_err());
        assert!("-1,1".parse::<Coords>().is_err());
        assert!("2,-10".parse::<Coords>().is_err());
    }
}
