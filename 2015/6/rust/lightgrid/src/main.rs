use nom::IResult;

#[allow(unused)]
fn parse_hello(input: &str) -> IResult<&str, &str, ()> {
    match input.strip_prefix("Hello") {
        Some(tail) => Ok((tail, "Hello")),
        None => Err(nom::Err::Error(())),
    }
}

#[test]
fn test() {
    assert_eq!(parse_hello("Hello, World!").unwrap(), (", World!", "Hello"));
}

fn main() {
    println!("Hello, world!");
}
