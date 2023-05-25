pub mod models;
mod parsers;
use std::collections::{HashMap, HashSet};

use log::{debug, error};
use models::{Connection, ConnectionGraph, CycleError, Expr, Node, SignalMap, Wire};

fn from_connections(connections: impl Iterator<Item = Connection>) -> ConnectionGraph {
    let mut graph = HashMap::new();
    for connection in connections {
        let target = connection.target;
        let dependencies = resolve_dependencies(&connection.source);
        graph.insert(
            target.clone(),
            Node {
                dependencies,
                expr: connection.source,
            },
        );
    }
    graph
}

pub fn run(lines: impl Iterator<Item = String>) -> Option<SignalMap> {
    signal_map(lines.map(|line| {
        let parts: Vec<&str> = line.splitn(2, '#').map(|l| l.trim()).collect();

        let connection = parts[0]
            .parse::<Connection>()
            .unwrap_or_else(|_| panic!("Error parsing line: {}", line));

        debug!("Parsed connection: {:?}", connection);

        connection
    }))
}

pub fn signal_map(connections: impl Iterator<Item = Connection>) -> Option<SignalMap> {
    let graph = from_connections(connections);
    let ts = topological_sort(&graph)?;
    debug!(
        "Found a following topological sorting for the connection graph: {:?}",
        ts
    );

    let mut output = HashMap::new();
    for wire in ts {
        let node = graph.get(&wire).unwrap();
        let value = evaluate(&output, &node.expr);
        output.insert(wire, value);
    }

    Some(output)
}

fn resolve_dependencies(expr: &Expr) -> HashSet<Wire> {
    match expr {
        Expr::Symbol(s) => HashSet::from([s.clone()]),
        Expr::Value(_) => HashSet::new(),
        Expr::Not(value) => resolve_dependencies(value),
        Expr::And(left, right)
        | Expr::Or(left, right)
        | Expr::LShift(left, right)
        | Expr::RShift(left, right) => {
            let l = resolve_dependencies(left);
            let r = resolve_dependencies(right);
            l.union(&r).cloned().collect()
        }
    }
}

fn topological_sort(graph: &ConnectionGraph) -> Option<Vec<Wire>> {
    let mut unmarked: HashSet<_> = graph.keys().collect();
    let mut temp: HashSet<&String> = HashSet::new();
    let mut perm: HashSet<&String> = HashSet::new();
    let mut path: Vec<String> = Vec::new();

    for node in unmarked.drain() {
        if dfs(graph, node, &mut temp, &mut perm, &mut path).is_err() {
            error!("Detected cycle in dependency graph. Circuit is not realizable.");
            return None;
        }
    }

    Some(path)
}

fn dfs<'a>(
    graph: &'a ConnectionGraph,
    node: &'a String,
    temp: &mut HashSet<&'a String>,
    perm: &mut HashSet<&'a String>,
    tlist: &mut Vec<String>,
) -> Result<(), CycleError> {
    if perm.contains(node) {
        return Ok(());
    }

    if temp.contains(node) {
        return Err(CycleError);
    }

    temp.insert(node);
    let dependencies = &graph.get(node).unwrap().dependencies;
    for dependency in dependencies.iter() {
        dfs(graph, dependency, temp, perm, tlist)?;
    }
    perm.insert(node);

    tlist.push(node.clone());

    Ok(())
}

fn evaluate(wire_map: &SignalMap, expr: &Expr) -> u16 {
    match expr {
        Expr::Value(v) => *v,
        Expr::Not(v) => {
            let a = evaluate(wire_map, v);
            !a
        }
        Expr::And(left, right) => {
            let a = evaluate(wire_map, left);
            let b = evaluate(wire_map, right);
            a & b
        }
        Expr::Or(left, right) => {
            let a = evaluate(wire_map, left);
            let b = evaluate(wire_map, right);
            a | b
        }
        Expr::LShift(left, right) => {
            let a = evaluate(wire_map, left);
            let b = evaluate(wire_map, right);
            a << b
        }
        Expr::RShift(left, right) => {
            let a = evaluate(wire_map, left);
            let b = evaluate(wire_map, right);
            a >> b
        }
        Expr::Symbol(s) => {
            let value = wire_map.get(s);
            match value {
                None => panic!("Could not evaluate symbol {}", s),
                Some(v) => *v,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_circuit() {
        let result = run(vec!["123 -> a".to_string()].into_iter()).unwrap();
        assert_eq!(result, HashMap::from([("a".to_string(), 123)]));
    }

    #[test]
    fn test_sample_circuit() {
        let connections = vec![
            "123 -> x",
            "456 -> y",
            "x AND y -> d",
            "x OR y -> e",
            "x LSHIFT 2 -> f",
            "y RSHIFT 2 -> g",
            "NOT x -> h",
            "NOT y -> i",
        ];

        let wire_map = signal_map(
            connections
                .into_iter()
                .map(|s| s.parse::<Connection>().unwrap()),
        );

        let expected: SignalMap = HashMap::from(
            [
                ("d", 72),
                ("e", 507),
                ("f", 492),
                ("g", 114),
                ("h", 65412),
                ("i", 65079),
                ("x", 123),
                ("y", 456),
            ]
            .map(|(s, v)| (Wire::from(s), v)),
        );

        assert_eq!(wire_map.unwrap(), expected);
    }

    #[test]
    fn test_dag() {
        let result = run(vec!["a -> a".to_string()].into_iter());
        assert!(result.is_none());
    }
}
