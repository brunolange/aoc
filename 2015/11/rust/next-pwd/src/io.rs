pub fn read_password() -> String {
    std::env::args().nth(1).expect("Need password")
}

pub fn number_of_passwords() -> usize {
    std::env::args()
        .nth(2)
        .unwrap_or("1".to_owned())
        .parse::<usize>()
        .unwrap()
}
