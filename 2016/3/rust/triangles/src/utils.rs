pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
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
