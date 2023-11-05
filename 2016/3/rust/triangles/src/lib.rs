pub fn can_make_triangle(a: usize, b: usize, c: usize) -> bool {
    a < (b + c) && b < (a + c) && c < (a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_make_triangle() {
        assert!(can_make_triangle(3, 4, 5));
        assert!(!can_make_triangle(5, 10, 25));
    }
}
