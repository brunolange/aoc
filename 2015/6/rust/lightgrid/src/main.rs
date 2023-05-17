use nom::{IResult, Parser};

#[allow(unused)]
fn parse_hello(input: &str) -> IResult<&str, &str, ()> {
    match input.strip_prefix("Hello") {
        Some(tail) => Ok((tail, "Hello")),
        None => Err(nom::Err::Error(())),
    }
}

#[allow(unused)]
fn parse_tag<'i: 't, 't>(tag: &'t str) -> impl Parser<&'i str, &'i str, ()> + 't {
    move |input: &'i str| match input.strip_prefix(tag) {
        Some(tail) => Ok((tail, &input[..tag.len()])),
        None => Err(nom::Err::Error(())),
    }
}

#[test]
fn test() {
    assert_eq!(parse_hello("Hello, World!").unwrap(), (", World!", "Hello"));
    assert_eq!(
        parse_tag("Hello").parse("Hello, World!").unwrap(),
        (", World!", "Hello")
    );
}

fn main() {
    println!("Hello, world!");
}
