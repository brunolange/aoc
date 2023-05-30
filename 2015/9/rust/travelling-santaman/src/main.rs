mod io;
use crate::io::lines;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Node {
    city: String,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Edge {
    distance: usize,
}

#[derive(Debug)]
struct Graph<'a> {
    adj: HashMap<&'a Node, HashMap<&'a Node, &'a Edge>>,
}

impl<'a> Graph<'a> {
    fn add_edge(&mut self, start: &'a Node, destination: &'a Node, edge: &'a Edge) {
        self.adj
            .entry(start)
            .or_insert(HashMap::new())
            .insert(destination, edge);
    }
}

fn parse_line(input: &str) -> Option<(Node, Edge, Node)> {
    let parts: Vec<&str> = input.split(" = ").collect();
    if parts.len() != 2 {
        return None;
    }

    let left = parts[0].trim();
    let right = parts[1].trim();

    let trip: Vec<&str> = left.split(" to ").collect();
    if trip.len() != 2 {
        return None;
    }

    let start = trip[0].trim();
    let destination = trip[1].trim();

    if start.len() == 0 || destination.len() == 0 {
        return None;
    }

    match right.parse::<usize>() {
        Err(_) => None,
        Ok(distance) => Some((
            Node {
                city: start.to_owned(),
            },
            Edge { distance },
            Node {
                city: destination.to_owned(),
            },
        )),
    }
}

fn main() {
    let mut graph = Graph {
        adj: HashMap::new(),
    };
    let xs = lines()
        .map(|line| parse_line(line.as_ref()).expect("Error parsing line"))
        .collect::<Vec<(Node, Edge, Node)>>();

    for (start, edge, destination) in xs.iter() {
        graph.add_edge(start, destination, edge);
        graph.add_edge(destination, start, edge);
    }

    let nodes = graph.adj.keys();
    let n = nodes.len();

    let hamiltonian_paths = nodes.permutations(n).filter(|permutation| {
        permutation
            .iter()
            .tuple_windows()
            .all(|(source, destination)| {
                let neighbors = graph.adj.get(**source).unwrap();
                neighbors.contains_key(**destination)
            })
    });

    let mut min_total_distance = usize::MAX;
    let mut optimal_path = vec![];
    for hamiltonian_path in hamiltonian_paths {
        let total_distance =
            hamiltonian_path
                .iter()
                .tuple_windows()
                .fold(0, |acc, (source, destination)| {
                    acc + graph
                        .adj
                        .get(**source)
                        .unwrap()
                        .get(**destination)
                        .unwrap()
                        .distance
                });
        println!(
            "Hamiltonian path: {:?} has total distance: {}",
            hamiltonian_path, total_distance
        );
        if total_distance < min_total_distance {
            optimal_path = hamiltonian_path;
            min_total_distance = total_distance;
        }
    }

    // println!("graph = {:?}", graph);
    println!("optimal_path = {:?}", optimal_path);
    println!("min_total_distance = {:?}", min_total_distance);
}
