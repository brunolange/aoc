mod utils;

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use utils::split_on_uppercase;

type Atom = String;

#[derive(Debug)]
struct Molecule {
    atoms: Vec<Atom>,
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

fn main() {
    // let molecule: Molecule = "HCaOCaliforniA".parse().unwrap();
    let molecule: Molecule = "HOH".parse().unwrap();
    let replacements = [("H", "HO"), ("H", "OH"), ("O", "HH")];
    let replacement_map: HashMap<&str, HashSet<&str>> =
        replacements
            .into_iter()
            .fold(HashMap::new(), |mut acc, curr| {
                let (atom, molecule) = curr;
                acc.entry(atom)
                    .or_insert_with(HashSet::new)
                    .insert(molecule);
                acc
            });

    println!("molecule = {:?}", molecule.atoms);
    println!("replacement_map = {:?}", replacement_map);
}
