use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1};
use nom::character::complete::{alpha1, digit1, space0};
use nom::combinator::{all_consuming, map_res, recognize};
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;
use std::str::FromStr;

pub type Wire = String;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Symbol(Wire),
    Value(u16),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    LShift(Box<Expr>, Box<Expr>),
    RShift(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
}

impl FromStr for Expr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, expr) = parse_expr(s).map_err(|_| ())?;
        Ok(expr)
    }
}

#[derive(Debug)]
pub struct Connection {
    pub source: Expr,
    pub target: Wire,
}

fn take_word(input: &str) -> IResult<&str, &str> {
    preceded(space0, recognize(take_till1(|c| c == ' ')))(input)
}

fn parse_value(input: &str) -> IResult<&str, u16> {
    all_consuming(map_res(digit1, str::parse))(input)
}

fn parse_symbol(input: &str) -> IResult<&str, &str> {
    // TODO: reject protected symbols
    all_consuming(alpha1)(input)
}

fn parse_not(input: &str) -> IResult<&str, Expr> {
    preceded(tag("NOT "), parse_expr)(input)
}

fn parse_binary_op(input: &str) -> IResult<&str, Expr> {
    let mut parser = tuple((
        take_word,
        alt((tag(" AND "), tag(" OR "), tag(" LSHIFT "), tag(" RSHIFT "))), // TODO: map from array
        take_word,
    ));

    let (_, (left, op, right)) = parser(input)?;
    let left = left.parse::<Expr>().unwrap(); // TODO: convert to nom's error
    let right = right.parse::<Expr>().unwrap(); // TODO: convert to nom's error
    let binary_op = match op.trim() {
        "AND" => Expr::And,
        "OR" => Expr::Or,
        "LSHIFT" => Expr::LShift,
        "RSHIFT" => Expr::LShift,
        _ => panic!("TODO: convert to nom's error"),
    };
    let expr = binary_op(Box::new(left), Box::new(right));

    Ok((input, expr))
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    if let Ok((input, value)) = parse_value(input) {
        return Ok((input, Expr::Value(value)));
    }

    if let Ok((input, value)) = parse_symbol(input) {
        return Ok((input, Expr::Symbol(Wire::from(value))));
    }

    if let Ok((input, expr)) = parse_not(input) {
        // let expr = expr.parse::<Expr>().unwrap(); // TODO: convert to nom's error
        return Ok((input, Expr::Not(Box::new(expr))));
    }

    parse_binary_op(input)
}

impl FromStr for Connection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_wire = take_word;
        let mut parser = all_consuming(separated_pair(parse_expr, tag(" -> "), parse_wire));

        let (_, (expr, wire)) = parser(s).unwrap(); // TODO: convert to nom's error
        Ok(Connection {
            source: expr,
            target: Wire::from(wire),
        })
    }
}

// fn _t() {
//     let _conn = Connection {
//         source: Expr::Value(123),
//         target: Wire::from("x"),
//     };

//     let _conn = Connection {
//         source: Expr::And(
//             Box::new(Expr::Symbol(Wire::from("x"))),
//             Box::new(Expr::Symbol(Wire::from("y"))),
//         ),
//         target: Wire::from("d"),
//     };
// }

// a circuit is an ordered list of connections
// 123 -> x
// 456 -> x
// is not the same as
// 456 -> x
// 123 -> x

// what if I want to support connections coming in any order
// for full parallelization
// transform connections into a dependency graph where I can run a topological sort
// to see if its even solvable.
// x AND y -> z
// 123 -> x
// ...
// 456 -> y  // which resolves z

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_symbol() {
        assert_eq!("x".parse::<Expr>().unwrap(), Expr::Symbol(Wire::from("x")));
        assert_eq!(
            "foobar".parse::<Expr>().unwrap(),
            Expr::Symbol(Wire::from("foobar"))
        );
        assert!("foo bar".parse::<Expr>().is_err());
    }

    #[test]
    fn test_expr_value() {
        assert_eq!("123".parse::<Expr>().unwrap(), Expr::Value(123));
        assert_eq!("65535".parse::<Expr>().unwrap(), Expr::Value(65535));
        assert!("65536".parse::<Expr>().is_err());
    }

    #[test]
    fn test_expr_not() {
        assert_eq!(
            "NOT x".parse::<Expr>().unwrap(),
            Expr::Not(Box::new(Expr::Symbol(Wire::from("x"))))
        );
        assert_eq!(
            "NOT 42".parse::<Expr>().unwrap(),
            Expr::Not(Box::new(Expr::Value(42)))
        );
        // assert!("NOT 65536".parse::<Expr>().is_err()); // can't run this yet, need to convert to nom's errors first
        assert_eq!(
            "NOT NOT x".parse::<Expr>().unwrap(),
            Expr::Not(Box::new(Expr::Not(Box::new(Expr::Symbol(Wire::from("x"))))))
        );
        assert_eq!(
            "NOT NOT 42".parse::<Expr>().unwrap(),
            Expr::Not(Box::new(Expr::Not(Box::new(Expr::Value(42)))))
        );
        assert_eq!(
            "NOT NOT NOT 99".parse::<Expr>().unwrap(),
            Expr::Not(Box::new(Expr::Not(Box::new(Expr::Not(Box::new(
                Expr::Value(99)
            ))))))
        );
    }

    #[test]
    fn test_expr_and() {
        assert_eq!(
            "x AND y".parse::<Expr>(),
            Ok(Expr::And(
                Box::new(Expr::Symbol(Wire::from("x"))),
                Box::new(Expr::Symbol(Wire::from("y")))
            ))
        );

        assert_eq!(
            "x AND 42".parse::<Expr>(),
            Ok(Expr::And(
                Box::new(Expr::Symbol(Wire::from("x"))),
                Box::new(Expr::Value(42))
            ))
        );

        assert_eq!(
            "100 AND 42".parse::<Expr>(),
            Ok(Expr::And(
                Box::new(Expr::Value(100)),
                Box::new(Expr::Value(42))
            ))
        );

        // assert_eq!(
        //     "x AND NOT y".parse::<Expr>(),
        //     Ok(Expr::And(
        //         Box::new(Expr::Symbol(Wire::from("x"))),
        //         Box::new(Expr::Not(Box::new(Expr::Symbol(Wire::from("y")))))
        //     ))
        // );
    }
}
