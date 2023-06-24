pub fn read_password() -> String {
    std::env::args().nth(1).expect("Need password")
}
