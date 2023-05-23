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
