use nom::IResult;

use decomp::Marker;

pub fn parser_marker(s: &str) -> IResult<&str, Marker> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_parser() {
        assert_eq!(
            "(10x2)".parse::<Marker>().unwrap(),
            Marker {
                take: 10,
                repeate: 2
            }
        )
    }
}
