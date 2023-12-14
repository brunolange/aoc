use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Marker {
    pub take: usize,
    pub repeate: usize,
}

#[derive(Debug)]
pub struct MarkerParsingError(String);

impl FromStr for Marker {
    type Err = MarkerParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
