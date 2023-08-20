use std::collections::{HashMap, HashSet};

use medicine::{Molecule, TransitionMap};

fn main() {
    // let molecule: Molecule = "HCaOCaliforniA".parse().unwrap();
    let molecule: Molecule = "HOH".parse().unwrap();
    let transition_map: TransitionMap = [("H", "HO"), ("H", "OH"), ("O", "HH")].into_iter().fold(
        HashMap::new(),
        |mut acc, curr| {
            let (atom, molecule) = curr;
            acc.entry(atom.to_owned())
                .or_insert_with(HashSet::new)
                .insert(molecule.to_owned());
            acc
        },
    );

    println!("molecule = {:?}", molecule.atoms);
    println!("transition_map = {:?}", transition_map);
}
