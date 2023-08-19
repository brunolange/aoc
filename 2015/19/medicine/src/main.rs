use std::collections::{HashMap, HashSet};

use medicine::Molecule;

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
