mod utils;

use crate::utils::split_on_uppercase;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub type Atom = String;
pub type TransitionMap = HashMap<Atom, HashSet<Molecule>>;

#[derive(Debug, Eq, PartialEq, Hash)]
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

pub struct MoleculeIter<'a> {
    transition_map: &'a TransitionMap,
    stack: Vec<&'a Molecule>,
}

impl<'a> Iterator for MoleculeIter<'a> {
    type Item = Vec<Molecule>;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().map(|curr| {
            curr.atoms
                .iter()
                .enumerate()
                .flat_map(|(i, atom)| {
                    self.transition_map.get(atom).map(|transitions| {
                        transitions
                            .iter()
                            .map(move |transition| {
                                let mut nxt = curr.atoms.clone();
                                let _ = nxt.splice(i..i + 1, transition.atoms.clone());
                                Molecule { atoms: nxt }
                            })
                            .into_iter()
                    })
                })
                .flatten()
                .collect()
        })
    }
}

impl<'a> Molecule {
    pub fn iter(&'a self, transition_map: &'a TransitionMap) -> MoleculeIter<'a> {
        MoleculeIter {
            transition_map,
            stack: vec![self],
        }
    }
}

// pub fn next_gen(
//     molecule: Molecule,
//     transition_map: TransitionMap,
// ) -> impl Iterator<Item = dyn Iterator<Item = Molecule>> {
//     let xs = molecule
//         .atoms
//         .iter()
//         .map(|atom| {
//             let molecules = transition_map.get(atom).unwrap();
//             molecules
//         })
//         .map(|x| x);

//     let w: Vec<Molecule> = ["CaliForNiA", "OReGoN"]
//         .into_iter()
//         .map(|s| s.parse::<Molecule>().unwrap())
//         .collect();

//     // vec![w.iter()].into_iter()
// }
