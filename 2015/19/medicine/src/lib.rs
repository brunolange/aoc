mod utils;

use crate::utils::split_on_uppercase;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub type Atom = String;

#[derive(Debug)]
pub struct Molecule {
    pub atoms: Vec<Atom>,
}

impl FromStr for Molecule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Molecule {
            atoms: split_on_uppercase(s)
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
        })
    }
}

pub type TransitionMap = HashMap<String, HashSet<String>>;
