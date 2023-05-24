mod models;
mod parsers;
use std::collections::{HashMap, HashSet};

use models::{Connection, Expr, Wire};

type WireMap = HashMap<Wire, u16>;
type G = HashMap<Wire, HashSet<Wire>>;

pub fn run(lines: impl Iterator<Item = String>) -> WireMap {
    reduce(lines.map(|s| s.parse::<Connection>().unwrap()))
}

pub fn reduce(connections: impl Iterator<Item = Connection>) -> WireMap {
    let mut graph: G = HashMap::new();
    // let mut co_graph: G = HashMap::new();

    for connection in connections {
        println!("evaluating connection: {:?}", connection);
        let target = connection.target;
        let dependencies = resolve_dependencies(&connection.source);

        println!(
            "target {} has these dependencies: {:?}",
            target, dependencies
        );

        graph.insert(target, dependencies);
        // for d in dependencies.into_iter() {
        //     co_graph.entry(d.clone()).or_default().insert(target);
        // }
        // let value = evaluate(&wire_map, connection.source);
        // wire_map.insert(connection.target, value);

        println!("graph = {:?}", graph);
    }

    // graph
    HashMap::new()
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

// fn evaluate(wire_map: &WireMap, expr: Expr) -> u16 {
//     match expr {
//         Expr::Value(v) => v,
//         Expr::Not(v) => {
//             let a = evaluate(wire_map, *v);
//             !a
//         }
//         Expr::And(left, right) => {
//             let a = evaluate(wire_map, *left);
//             let b = evaluate(wire_map, *right);
//             a & b
//         }
//         Expr::Or(left, right) => {
//             let a = evaluate(wire_map, *left);
//             let b = evaluate(wire_map, *right);
//             a | b
//         }
//         Expr::LShift(left, right) => {
//             let a = evaluate(wire_map, *left);
//             let b = evaluate(wire_map, *right);
//             a << b
//         }
//         Expr::RShift(left, right) => {
//             let a = evaluate(wire_map, *left);
//             let b = evaluate(wire_map, *right);
//             a >> b
//         }
//         Expr::Symbol(s) => {
//             let value = wire_map.get(&s);
//             match value {
//                 None => panic!("Could not evaluate symbol {}", s),
//                 Some(v) => *v,
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit() {
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

        let wire_map = reduce(
            connections
                .into_iter()
                .map(|s| s.parse::<Connection>().unwrap()),
        );

        let expected: WireMap = HashMap::from(
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

        assert_eq!(wire_map, expected);
    }
}
