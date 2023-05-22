use crate::models::{Gate, Wire};

pub fn apply(gate: Gate) -> Wire {
    match gate {
        Gate::AND(a, b) => a & b,
        Gate::OR(a, b) => a | b,
        Gate::NOT(a) => !a,
        Gate::LSHIFT(a, l) => a << l,
        Gate::RSHIFT(a, r) => a >> r,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_and() {
        assert_eq!(apply(Gate::AND(0, 0)), 0);
        assert_eq!(apply(Gate::AND(0, 1)), 0);
        assert_eq!(apply(Gate::AND(1, 0)), 0);
        assert_eq!(apply(Gate::AND(1, 1)), 1);
        assert_eq!(apply(Gate::AND(23, 39)), 7);
    }

    #[test]
    fn test_apply_or() {
        assert_eq!(apply(Gate::OR(0, 0)), 0);
        assert_eq!(apply(Gate::OR(0, 1)), 1);
        assert_eq!(apply(Gate::OR(1, 0)), 1);
        assert_eq!(apply(Gate::OR(1, 1)), 1);
        assert_eq!(apply(Gate::OR(23, 39)), 55);
    }

    #[test]
    fn test_apply_not() {
        assert_eq!(apply(Gate::NOT(0)), 65535);
        assert_eq!(apply(Gate::NOT(65535)), 0);
        assert_eq!(apply(Gate::NOT(1)), 65534);
        assert_eq!(apply(Gate::NOT(39)), 65496);
    }

    #[test]
    fn test_apply_lshift() {
        assert_eq!(apply(Gate::LSHIFT(0, 1)), 0);
        assert_eq!(apply(Gate::LSHIFT(0, 3)), 0);
        assert_eq!(apply(Gate::LSHIFT(1, 0)), 1);
        assert_eq!(apply(Gate::LSHIFT(1, 1)), 2);
        assert_eq!(apply(Gate::LSHIFT(1, 2)), 4);
        assert_eq!(apply(Gate::LSHIFT(1, 3)), 8);
        assert_eq!(apply(Gate::LSHIFT(23, 7)), 23 * 2 * 2 * 2 * 2 * 2 * 2 * 2);
    }

    #[test]
    fn test_apply_rshift() {
        assert_eq!(apply(Gate::RSHIFT(0, 1)), 0);
        assert_eq!(apply(Gate::RSHIFT(0, 3)), 0);
        assert_eq!(apply(Gate::RSHIFT(1, 0)), 1);
        assert_eq!(apply(Gate::RSHIFT(1, 1)), 0);
        assert_eq!(apply(Gate::RSHIFT(1, 2)), 0);
        assert_eq!(apply(Gate::RSHIFT(2, 1)), 1);
        assert_eq!(apply(Gate::RSHIFT(62, 3)), 7);
    }
}
