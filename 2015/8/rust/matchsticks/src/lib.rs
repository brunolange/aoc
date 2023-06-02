use std::collections::HashSet;

mod parsers;

use parsers::parse_count;

pub fn counts(line: &str) -> (usize, usize) {
    let (_, count) = parse_count(line).expect("Invalid input");
    (line.len(), count)
}

pub fn encode_counts(input: &str) -> (usize, usize) {
    let to_escape = HashSet::from(['"', '\\']);
    let extra = input.chars().filter(|c| to_escape.contains(c)).count();
    return (input.len() + 2 + extra, input.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts() {
        assert_eq!(counts("\"\""), (2, 0));
        assert_eq!(counts("\"abc\""), (5, 3));
        assert_eq!(counts("\"aaa\\\"aaa\""), (10, 7));
        assert_eq!(counts("\"\\x27\""), (6, 1));
    }
}
