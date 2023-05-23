use std::str::FromStr;

use crate::parsers::{parse_connection, parse_expr};

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

impl FromStr for Connection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, expr) = parse_connection(s).map_err(|_| ())?;
        Ok(expr)
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
