use std::collections::{HashMap, HashSet};

use medicine::{Molecule, TransitionMap};

fn main() {
    let molecule: Molecule = "HONH".parse().unwrap();
    let transition_map: TransitionMap = [("H", "HO"), ("H", "OH"), ("O", "HH")].into_iter().fold(
        HashMap::new(),
        |mut acc, curr| {
            let (atom, mol) = curr;
            let molecule: Molecule = mol.parse().unwrap();
            acc.entry(atom.to_owned())
                .or_insert_with(HashSet::new)
                .insert(molecule);
            acc
        },
    );

    println!("molecule = {:?}", molecule.atoms);
    println!("transition_map = {:?}", transition_map);

    // for molecules in next_gen(molecule, transition_map) {
    //     for molecule in molecules {
    //         println!("molecule = {:?}", molecule.atoms);
    //     }
    // }
    for molecules in molecule.iter(&transition_map) {
        println!("molecules = {:?}", molecules);
    }
}
