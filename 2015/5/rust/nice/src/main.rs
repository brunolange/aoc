fn main() {
    println!("Hello, world!");
}

#[allow(unused)]
fn is_nice_string(text: &str) -> bool {
    todo!()
}

#[test]
fn test_nice_strings() {
    for nice in vec![
        "ugknbfddgicrmopn",
        "aaa",
    ] {
        assert_eq!(is_nice_string(nice), true);
    }
}

#[test]
fn test_naughty_strings() {
    for naughty in vec![
        "jchzalrnumimnmhp",
        "haegwjzuvuyypxyu",
        "dvszwmarrgswjxmb",
    ] {
        assert_eq!(is_nice_string(naughty), false);
    }
}
