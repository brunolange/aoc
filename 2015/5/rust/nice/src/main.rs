fn main() {
    println!("Hello, world!");
}

#[allow(unused)]
fn is_nice_string(text: &str) -> bool {
    todo!()
}

#[test]
fn test_nice_strings() {
    for nice_string in vec![
        "ugknbfddgicrmopn",
    ] {
        assert_eq!(is_nice_string(nice_string), true);
    }
}