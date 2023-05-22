#[derive(Debug, PartialEq)]
pub struct Circuit {}

impl Circuit {
    pub fn from_instructions(instructions: impl Iterator<Item = String>) -> Self {
        Circuit {}
    }
}

#[derive(Debug, PartialEq)]
pub struct CircuitError(pub String);
