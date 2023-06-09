use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1, take_until};
use nom::character::complete::{alpha1, digit1, multispace0, multispace1, space0};
use nom::combinator::{all_consuming, map_res, recognize, rest};
use nom::sequence::{delimited, preceded, separated_pair, tuple};
use nom::IResult;

use crate::models::{Connection, Expr, Wire};

pub fn parse_connection(input: &str) -> IResult<&str, Connection> {
    let (remaining, (expr, wire)) = all_consuming(separated_pair(
        parse_expr,
        tuple((multispace0, tag("->"), multispace0)),
        alpha1,
    ))(input)?;
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
        parse_binary_gate("OR", |l, r| Expr::Or(Box::new(l), Box::new(r))),
        parse_binary_gate("AND", |l, r| Expr::And(Box::new(l), Box::new(r))),
        parse_binary_gate("LSHIFT", |l, r| Expr::LShift(Box::new(l), Box::new(r))),
        parse_binary_gate("RSHIFT", |l, r| Expr::RShift(Box::new(l), Box::new(r))),
        parse_not,
        parse_value,
        parse_symbol,
    ))(input)
}

pub fn take_word(input: &str) -> IResult<&str, &str> {
    preceded(space0, recognize(take_till1(|c| c == ' ')))(input)
}

fn parse_value(input: &str) -> IResult<&str, Expr> {
    let (remaining, value) = map_res(digit1, str::parse)(input)?;
    Ok((remaining, Expr::Value(value)))
}

fn parse_symbol(input: &str) -> IResult<&str, Expr> {
    // TODO: reject protected symbols
    let (remaining, value) = alpha1(input)?;
    Ok((remaining, Expr::Symbol(Wire::from(value))))
}

fn parse_not(input: &str) -> IResult<&str, Expr> {
    let (remaining, expr) = preceded(tag("NOT "), parse_expr)(input)?;
    Ok((remaining, Expr::Not(Box::new(expr))))
}

/// TODO: review leaves and nodes. leaves are symbols and values. maximally reduced. others are nodes.

fn _parse_left_right(marker: &str) -> impl Fn(&str) -> IResult<&str, (&str, &str)> + '_ {
    move |input: &str| {
        let (input, left) = take_until(marker)(input)?;
        let (input, _) = tag(marker)(input)?;
        let (input, right) = rest(input)?;
        Ok((input, (left, right)))
    }
}

fn parse_binary_gate<'a, F>(
    marker: &'a str,
    binary_gate: F,
) -> impl Fn(&str) -> IResult<&str, Expr> + 'a
where
    F: Fn(Expr, Expr) -> Expr + 'a,
{
    move |input| {
        let (remaining, (left, right)) = separated_pair(
            take_word,
            delimited(multispace1, tag(marker), multispace1),
            take_word,
        )(input)?;

        let (_, left) = parse_expr(left)?;
        let (_, right) = parse_expr(right)?;

        Ok((remaining, binary_gate(left, right)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_connection() {
        assert_eq!(
            "123 -> x".parse::<Connection>().unwrap(),
            Connection {
                source: Expr::Value(123),
                target: Wire::from("x")
            }
        );

        assert_eq!(
            "x AND y -> d".parse::<Connection>().unwrap(),
            Connection {
                source: Expr::And(
                    Box::new(Expr::Symbol(Wire::from("x"))),
                    Box::new(Expr::Symbol(Wire::from("y")))
                ),
                target: Wire::from("d")
            }
        );

        assert_eq!(
            "y RSHIFT 2 -> g".parse::<Connection>().unwrap(),
            Connection {
                source: Expr::RShift(
                    Box::new(Expr::Symbol(Wire::from("y"))),
                    Box::new(Expr::Value(2))
                ),
                target: Wire::from("g")
            }
        );

        assert!("NOT x -> 2".parse::<Connection>().is_err());
    }

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

    // #[test]
    // fn test_parens() {
    //     assert_eq!(
    //         "(NOT x) AND y".parse::<Expr>(),
    //         Ok(Expr::And(
    //             Box::new(Expr::Not(Box::new(Expr::Symbol(Wire::from("x"))))),
    //             Box::new(Expr::Symbol(Wire::from("y")))
    //         ))
    //     );

    //     assert_eq!(
    //         "NOT (x AND y)".parse::<Expr>(),
    //         Ok(Expr::Not(Box::new(Expr::And(
    //             Box::new(Expr::Symbol(Wire::from("x"))),
    //             Box::new(Expr::Symbol(Wire::from("y")))
    //         ))))
    //     );
    // }
}
