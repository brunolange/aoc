pub fn seed() -> String {
    std::env::args().nth(1).unwrap_or("1113122113".to_owned())
}

pub fn iterations() -> usize {
    std::env::args()
        .nth(2)
        .unwrap_or("40".to_owned())
        .parse()
        .unwrap()
}
