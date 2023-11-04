mod io;

fn can_make_triangle(a: usize, b: usize, c: usize) -> bool {
    a < (b + c) && b < (a + c) && c < (a + b)
}

fn main() {
    let count = io::lines()
        .map(io::read)
        .filter(|(a, b, c)| can_make_triangle(*a, *b, *c))
        .count();

    println!("count = {count}");
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
