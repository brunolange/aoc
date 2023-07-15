// Solutions to a_1 + a_2 + ... + a_n = s
// where a_i and n are natural numbers
pub fn f(s: usize, n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![];
    }
    if n == 1 {
        return vec![vec![s]];
    }

    (0..=s)
        .flat_map(|i| {
            f(s - i, n - 1).into_iter().map(move |mut prev| {
                prev.push(i);
                prev
            })
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_f() {
        assert_eq!(f(0, 1), [[0]]);

        assert_eq!(f(1, 1), [[1]]);
        assert_eq!(f(99, 1), [[99]]);

        assert_eq!(f(3, 2), [[3, 0], [2, 1], [1, 2], [0, 3]]);
        assert_eq!(
            f(4, 3),
            [
                [4, 0, 0],
                [3, 1, 0],
                [2, 2, 0],
                [1, 3, 0],
                [0, 4, 0],
                [3, 0, 1],
                [2, 1, 1],
                [1, 2, 1],
                [0, 3, 1],
                [2, 0, 2],
                [1, 1, 2],
                [0, 2, 2],
                [1, 0, 3],
                [0, 1, 3],
                [0, 0, 4]
            ]
        );
    }
}
