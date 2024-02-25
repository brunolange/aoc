use std::collections::{HashMap, HashSet, LinkedList};

use itertools::Itertools;
use md5;

fn triplets(hash: &str) -> HashSet<char> {
    hash.chars()
        .into_iter()
        .tuple_windows()
        .filter(|(a, b, c)| a == b && b == c)
        .map(|(a, _, _)| a)
        .collect()
}

fn quints(hash: &str) -> HashSet<char> {
    hash.chars()
        .into_iter()
        .tuple_windows()
        .filter(|(a, b, c, d, e)| a == b && b == c && c == d && d == e)
        .map(|(a, _, _, _, _)| a)
        .collect()
}

fn main() {
    let salt = "abc";

    // let mut tm: HashMap<char, LinkedList<usize>> = HashMap::new(); // TODO: keep a short list of candidates. 1000 max but in reality much smaller.
    let mut triplet_map: HashMap<char, Vec<i32>> = HashMap::new();
    let mut quint_map: HashMap<char, Vec<i32>> = HashMap::new();

    for i in 0.. {
        let nxt = format!("{}{}", salt, i);
        let hash = format!("{:x}", md5::compute(nxt.clone()));
        println!("{nxt} -> {hash}");
        for c in quints(&hash) {
            quint_map
                .entry(c)
                .and_modify(|v| v.push(i))
                .or_insert_with(|| vec![i]);

            let matches = triplet_map.entry(c).or_default();
            if matches.into_iter().rev().any(|start| i - *start <= 1000) {
                panic!("{i}: {nxt} | {hash} | {:?}", matches);
            }
        }
        for c in triplets(&hash) {
            triplet_map
                .entry(c)
                .and_modify(|v| v.push(i))
                .or_insert_with(|| vec![i]);
        }
    }
}
