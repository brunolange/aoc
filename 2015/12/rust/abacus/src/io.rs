pub fn input() -> String {
    let path = std::env::args().nth(1).expect("need file path");
    std::fs::read_to_string(path).expect("error reading file")
}
