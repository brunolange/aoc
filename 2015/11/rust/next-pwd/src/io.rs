pub fn read_password() -> String {
    std::env::args().nth(1).expect("Need password")
}

pub fn fast() -> bool {
    std::env::var("FAST").unwrap_or_default() != String::default()
}
