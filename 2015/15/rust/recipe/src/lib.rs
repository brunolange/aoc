use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Ingredient {
    pub name: String,
    pub capacity: i64,
    pub durability: i64,
    pub flavor: i64,
    pub texture: i64,
    pub calories: i64,
}

#[derive(Debug)]
pub struct Amount<'a> {
    pub quantity: usize,
    pub ingredient: &'a Ingredient,
}

pub fn score(amounts: &[Amount]) -> usize {
    amounts
        .iter()
        .map(|a| {
            let ing = a.ingredient;
            let q = a.quantity;
            [ing.capacity, ing.durability, ing.flavor, ing.texture].map(|v| v * q as i64)
        })
        .reduce(|acc, curr| {
            acc.into_iter()
                .zip(curr)
                .map(|(a, b)| a + b)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .unwrap()
        .into_iter()
        .map(|v| std::cmp::max(0, v) as usize)
        // .fold(1, |acc, curr| acc * curr) clippy FTW!
        .product()
}

///
/// Non-negative integer solutions to $x_1 + x_2 + ... + x_k = n$.
/// That is, the k-weak composition of n.
///
/// # Example
/// ```rust
/// use recipe::n_multichoose_k;
/// let empty: Vec<Vec<usize>> = vec![];
/// assert_eq!(n_multichoose_k(100, 0), empty);
/// assert_eq!(n_multichoose_k(100, 1), [[100]]);
/// assert_eq!(n_multichoose_k(3, 2), [[3, 0], [2, 1], [1, 2], [0, 3]]);
/// ```
///
pub fn n_multichoose_k(n: usize, k: usize) -> Vec<Vec<usize>> {
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
        assert_eq!(n_multichoose_k(0, 1), [[0]]);

        assert_eq!(n_multichoose_k(1, 1), [[1]]);
        assert_eq!(n_multichoose_k(99, 1), [[99]]);

        assert_eq!(n_multichoose_k(3, 2), [[3, 0], [2, 1], [1, 2], [0, 3]]);
        assert_eq!(
            n_multichoose_k(4, 3),
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
