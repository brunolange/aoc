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

#[allow(unused)]
fn parse_comma_tags<'i: 't, 't>(
    tag1: &'t str,
    tag2: &'t str,
) -> impl Parser<&'i str, (&'i str, &'i str), ()> + 't {
    move |input: &'i str| {
        let (tail, value1) = parse_tag(tag1).parse(input)?;
        let (tail, _) = parse_tag(", ").parse(tail)?;
        let (tail, value2) = parse_tag(tag2).parse(tail)?;

        Ok((tail, (value1, value2)))
    }
}

#[test]
fn test() {
    assert_eq!(parse_hello("Hello, World!").unwrap(), (", World!", "Hello"));
    assert_eq!(
        parse_tag("Hello").parse("Hello, World!").unwrap(),
        (", World!", "Hello")
    );
    assert_eq!(
        parse_comma_tags("Hello", "World")
            .parse("Hello, World!")
            .unwrap(),
        ("!", ("Hello", "World"))
    )
}

fn main() {
    println!("Hello, world!");
}
