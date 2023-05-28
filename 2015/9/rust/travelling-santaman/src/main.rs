mod io;

use std::collections::HashMap;

use crate::io::lines;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Node {
    city: String,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Edge {
    distance: usize,
}

#[derive(Debug)]
struct Graph {
    adj: HashMap<Node, HashMap<Edge, Node>>,
}

impl Graph {
    fn add_edge(&mut self, start: Node, edge: Edge, destination: Node) {
        self.adj
            .entry(start)
            .or_insert(HashMap::new())
            .insert(edge, destination);
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
    for line in lines() {
        let (start, edge, destination) = parse_line(line.as_ref()).expect("Error parsing line");
        graph.add_edge(start, edge, destination);
        println!("graph = {:?}", graph);
    }
}
