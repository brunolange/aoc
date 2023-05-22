mod models;
use std::collections::HashMap;

use models::{Connection, Expr, Wire};

pub fn reduce<'a>(connections: impl Iterator<Item = &'a Connection>) -> HashMap<&'a Wire, Expr> {
    let mut state: HashMap<&'a Wire, Expr> = HashMap::new();
    for connection in connections {
        println!("need to process connection: {:?}", connection);
        let target = &connection.target;
        state.insert(target, Expr::Value(0));
    }

    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit() {
        // let instructions = vec![
        //     "123 -> x",
        //     "456 -> y",
        //     "x AND y -> d",
        //     "x OR y -> e",
        //     "x LSHIFT 2 -> f",
        //     "y RSHIFT 2 -> g",
        //     "NOT x -> h",
        //     "NOT y -> i",
        // ]
        // .into_iter()
        // .map(str::to_string);

        // let connections = vec![Connection {
        //     source: Expr::Value(123),
        //     target: String::from("x"),
        // }];

        // let state = reduce(connections.iter());
        // let value = state.get(&String::from("x")).unwrap();

        // assert_eq!(*value, Expr::Value(123));

        // let answer: HashMap<_, Expr> = HashMap::from([
        //     ("d", Expr::Value(72)),
        //     ("e", Expr::Value(507)),
        //     ("f", Expr::Value(492)),
        //     ("g", Expr::Value(114)),
        //     ("h", Expr::Value(65412)),
        //     ("i", Expr::Value(65079)),
        //     ("x", Expr::Value(123)),
        //     ("y", Expr::Value(456)),
        // ]);

        // assert_eq!(reduce(connections.iter()), answer);
    }
}
