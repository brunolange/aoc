use std::collections::HashMap;

// Solutions to a_1 + a_2 + ... + a_n = s
// where a_i and n are natural numbers
pub fn f(s: usize, n: usize) -> Vec<Vec<usize>> {
    let mut memo: HashMap<(usize, usize), Vec<Vec<usize>>> = HashMap::new();
    fprime(s, n, &mut memo)
}

// Memoized version
fn fprime(
    s: usize,
    n: usize,
    memo: &mut HashMap<(usize, usize), Vec<Vec<usize>>>,
) -> Vec<Vec<usize>> {
    let key = (s, n);
    if let Some(payload) = memo.get(&key) {
        return payload.to_owned();
    }
    let payload = match n {
        0 => vec![],
        1 => vec![vec![s]],
        _ => (0..=s)
            .flat_map(|i| {
                fprime(s - i, n - 1, memo).into_iter().map(move |mut prev| {
                    prev.push(i);
                    prev
                })
            })
            .collect(),
    };

    memo.insert(key, payload.clone());

    payload
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
