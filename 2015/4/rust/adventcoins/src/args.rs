use clap::{builder::PossibleValue,Parser,ValueEnum};

#[derive(Debug, Clone)]
pub enum Strategy {
    Sequential,
    Parallel
}

impl ValueEnum for Strategy {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Sequential, Self::Parallel]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Sequential => PossibleValue::new("sequential").help("use sequential strategy"),
            Self::Parallel => PossibleValue::new("parallel").help("use parallel strategy"),
        })
    }
}

#[derive(Debug,Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub message: String,
    pub leading_zeros: usize,
    pub strategy: Option<Strategy>,
}