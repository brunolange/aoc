use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1};
use nom::character::complete::{alpha1, digit1, multispace1, space0};
use nom::combinator::{all_consuming, map_res, recognize};
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;

use crate::models::{Connection, Expr, Wire};

pub fn parse_connection(input: &str) -> IResult<&str, Connection> {
    let (remaining, (expr, wire)) =
        all_consuming(separated_pair(parse_expr, tag(" -> "), take_word))(input)?;
    Ok((
        remaining,
        Connection {
            source: expr,
            target: Wire::from(wire),
        },
    ))
}

pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_value,
        parse_symbol,
        parse_not,
        parse_binary_gate("AND", |l, r| Expr::And(Box::new(l), Box::new(r))),
        parse_binary_gate("OR", |l, r| Expr::Or(Box::new(l), Box::new(r))),
        parse_binary_gate("LSHIFT", |l, r| Expr::LShift(Box::new(l), Box::new(r))),
        parse_binary_gate("RSHIFT", |l, r| Expr::RShift(Box::new(l), Box::new(r))),
    ))(input)
}

pub fn take_word(input: &str) -> IResult<&str, &str> {
    preceded(space0, recognize(take_till1(|c| c == ' ')))(input)
}

fn parse_value(input: &str) -> IResult<&str, Expr> {
    let (remaining, value) = all_consuming(map_res(digit1, str::parse))(input)?;
    Ok((remaining, Expr::Value(value)))
}

fn parse_symbol(input: &str) -> IResult<&str, Expr> {
    // TODO: reject protected symbols
    let (remaining, value) = all_consuming(alpha1)(input)?;
    Ok((remaining, Expr::Symbol(Wire::from(value))))
}

fn parse_not(input: &str) -> IResult<&str, Expr> {
    let (remaining, expr) = preceded(tag("NOT "), parse_expr)(input)?;
    Ok((remaining, Expr::Not(Box::new(expr))))
}

/// TODO: review leaves and nodes. leaves are symbols and values. maximally reduced. others are nodes.

fn parse_binary_gate<'a, F>(
    marker: &'a str,
    binary_gate: F,
) -> impl Fn(&str) -> IResult<&str, Expr> + 'a
where
    F: Fn(Expr, Expr) -> Expr + 'a,
{
    move |input| {
        let (remaining, (left, right)) = separated_pair(
            take_word, // TODO: turn this into take anything, recurse, profit
            tuple((multispace1, tag(marker), multispace1)),
            take_word,
        )(input)?;

        let left = left.parse::<Expr>().unwrap(); // TODO: map_err + ?
        let right = right.parse::<Expr>().unwrap();

        Ok((remaining, binary_gate(left, right)))
    }
}

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
    }
    #[test]

    fn test_expr_or() {
        assert_eq!(
            "x OR y".parse::<Expr>(),
            Ok(Expr::Or(
                Box::new(Expr::Symbol(Wire::from("x"))),
                Box::new(Expr::Symbol(Wire::from("y")))
            ))
        );

        assert_eq!(
            "x OR 42".parse::<Expr>(),
            Ok(Expr::Or(
                Box::new(Expr::Symbol(Wire::from("x"))),
                Box::new(Expr::Value(42))
            ))
        );

        assert_eq!(
            "100 OR 42".parse::<Expr>(),
            Ok(Expr::Or(
                Box::new(Expr::Value(100)),
                Box::new(Expr::Value(42))
            ))
        );
    }

    #[test]
    fn test_expr_lshift() {
        assert_eq!(
            "x LSHIFT y".parse::<Expr>(),
            Ok(Expr::LShift(
                Box::new(Expr::Symbol(Wire::from("x"))),
                Box::new(Expr::Symbol(Wire::from("y")))
            ))
        );

        assert_eq!(
            "x LSHIFT 42".parse::<Expr>(),
            Ok(Expr::LShift(
                Box::new(Expr::Symbol(Wire::from("x"))),
                Box::new(Expr::Value(42))
            ))
        );

        assert_eq!(
            "100 LSHIFT 42".parse::<Expr>(),
            Ok(Expr::LShift(
                Box::new(Expr::Value(100)),
                Box::new(Expr::Value(42))
            ))
        );
    }

    #[test]
    fn test_expr_rshift() {
        assert_eq!(
            "x RSHIFT y".parse::<Expr>(),
            Ok(Expr::RShift(
                Box::new(Expr::Symbol(Wire::from("x"))),
                Box::new(Expr::Symbol(Wire::from("y")))
            ))
        );

        assert_eq!(
            "x RSHIFT 42".parse::<Expr>(),
            Ok(Expr::RShift(
                Box::new(Expr::Symbol(Wire::from("x"))),
                Box::new(Expr::Value(42))
            ))
        );

        assert_eq!(
            "100 RSHIFT 42".parse::<Expr>(),
            Ok(Expr::RShift(
                Box::new(Expr::Value(100)),
                Box::new(Expr::Value(42))
            ))
        );
    }
}
