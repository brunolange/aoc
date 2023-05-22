mod models;
use std::collections::HashMap;

use models::{Circuit, CircuitError};

pub fn solve(circuit: &Circuit) -> Result<HashMap<&str, usize>, CircuitError> {
    Err(CircuitError("under construction...".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn test_circuit() {
        let instructions = vec![
            "123 -> x",
            "456 -> y",
            "x AND y -> d",
            "x OR y -> e",
            "x LSHIFT 2 -> f",
            "y RSHIFT 2 -> g",
            "NOT x -> h",
            "NOT y -> i",
        ]
        .into_iter()
        .map(str::to_string);

        let circuit = Circuit::from_instructions(instructions);
        let answer: HashMap<_, usize> = HashMap::from([
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ]);

        assert_eq!(solve(&circuit), Ok(answer));
    }
}
