pub fn can_make_triangle(a: usize, b: usize, c: usize) -> bool {
    a < (b + c) && b < (a + c) && c < (a + b)
}

pub type Point = (f64, f64);

pub fn vertice(a: usize, b: usize, c: usize) -> Point {
    let a = a as f64;
    let b = b as f64;
    let c = c as f64;
    (
        (a.powi(2) + c.powi(2) - b.powi(2)) / (2.0 * a),
        (c.powi(2)
            - (a.powi(4)
                + b.powi(4)
                + c.powi(4)
                + 2.0 * ((a * c).powi(2) - (a * b).powi(2) - (b * c).powi(2)))
                / (4.0 * a.powi(2)))
        .sqrt(),
    )
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
