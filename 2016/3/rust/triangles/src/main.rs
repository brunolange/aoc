use itertools::Itertools;

mod io;

fn can_make_triangle(a: usize, b: usize, c: usize) -> bool {
    a < (b + c) && b < (a + c) && c < (a + b)
}

fn main() {
    let lines = io::lines();

    let count = match io::part() {
        io::Part::One => lines
            .map(io::read_k::<3>)
            .filter(|[a, b, c]| can_make_triangle(*a, *b, *c))
            .count(),

        io::Part::Two => lines
            .map(io::read_n)
            .chunks(3)
            .into_iter()
            .map(|rows| rows.collect::<Vec<Vec<usize>>>())
            .flat_map(transpose)
            .filter(|sides| {
                let [a, b, c]: [usize; 3] = sides[..].try_into().unwrap();
                can_make_triangle(a, b, c)
            })
            .count(),
    };

    println!("count = {count}");
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if v.is_empty() {
        return v;
    }

    let n = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|row| row.into_iter()).collect();

    (0..n)
        .map(|_| {
            iters
                .iter_mut()
                .map(|row| row.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
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
