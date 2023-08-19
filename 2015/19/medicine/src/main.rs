use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

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
    let molecule: Molecule = "HCaOCaliforniA".parse().unwrap();
    // let molecule = "HOH";
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

fn split_on_uppercase(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;

    for (i, c) in s.char_indices() {
        if c.is_ascii_uppercase() {
            if start < i {
                result.push(&s[start..i]);
            }
            start = i;
        }
    }

    if start < s.len() {
        result.push(&s[start..]);
    }

    result
}
