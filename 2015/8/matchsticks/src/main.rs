fn main() {
    println!("Hello, world!");
}

#[allow(unused)]
fn counts(line: &str) -> (usize, usize) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts() {
        assert_eq!(counts(""), (2, 0));
    }
}
