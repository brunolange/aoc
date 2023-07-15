use std::collections::HashMap;

///
/// Non-negative integer solutions to $x_1 + x_2 + ... + x_k = n$.
/// That is, the k-weak composition of n.
///
/// # Example
/// ```rust
/// use recipe::multisubsets;
/// let empty: Vec<Vec<usize>> = vec![];
/// assert_eq!(multisubsets(100, 0), empty);
/// assert_eq!(multisubsets(100, 1), [[100]]);
/// assert_eq!(multisubsets(3, 2), [[3, 0], [2, 1], [1, 2], [0, 3]]);
/// ```
///
pub fn multisubsets(n: usize, k: usize) -> Vec<Vec<usize>> {
    fn _multisubsets(
        n: usize,
        k: usize,
        memo: &mut HashMap<(usize, usize), Vec<Vec<usize>>>,
    ) -> Vec<Vec<usize>> {
        let key = (n, k);
        if let Some(payload) = memo.get(&key) {
            return payload.to_owned();
        }
        let payload = match k {
            0 => vec![],
            1 => vec![vec![n]],
            _ => (0..=n)
                .flat_map(|i| {
                    _multisubsets(n - i, k - 1, memo)
                        .into_iter()
                        .map(move |mut prev| {
                            prev.push(i);
                            prev
                        })
                })
                .collect(),
        };

        memo.insert(key, payload.clone());

        payload
    }

    let mut memo: HashMap<(usize, usize), Vec<Vec<usize>>> = HashMap::new();

    _multisubsets(n, k, &mut memo)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_f() {
        assert_eq!(multisubsets(0, 1), [[0]]);

        assert_eq!(multisubsets(1, 1), [[1]]);
        assert_eq!(multisubsets(99, 1), [[99]]);

        assert_eq!(multisubsets(3, 2), [[3, 0], [2, 1], [1, 2], [0, 3]]);
        assert_eq!(
            multisubsets(4, 3),
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
