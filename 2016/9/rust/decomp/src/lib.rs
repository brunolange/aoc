mod parsers;

use std::{fmt::Display, str::FromStr};

use nom::{
    bytes::complete::tag,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
use parsers::{parse_usize, take_anything_until};

#[derive(Debug, PartialEq)]
struct Marker {
    take: usize,
    repeat: usize,
}

fn parse_marker(s: &str) -> IResult<&str, Marker> {
    let (s, (take, repeat)) = delimited(
        tag("("),
        separated_pair(parse_usize, tag("x"), parse_usize),
        tag(")"),
    )(s)?;

    Ok((s, Marker { take, repeat }))
}

impl FromStr for Marker {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, marker) = parse_marker(s).map_err(|_| ())?;

        Ok(marker)
    }
}

#[derive(Debug, PartialEq)]
struct Segment<'a> {
    marker: Marker,
    text: &'a str,
}

impl<'a> Display for Segment<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let up_to = self.marker.take.min(self.text.len());
        write!(
            f,
            "{}{}",
            &self.text[..up_to].repeat(self.marker.repeat),
            &self.text[up_to..]
        )
    }
}

fn segments(s: &str) -> Vec<Segment> {
    let mut s2 = s;
    let mut output = Vec::new();

    while s2.len() != 0 {
        if let Ok((rest, marker)) = parse_marker.parse(s2) {
            let index = marker.take.min(rest.len());
            if let Ok((_, (excess, _))) = take_anything_until(parse_marker).parse(&rest[index..]) {
                let text = &rest[..(index + excess.len())];
                output.push(Segment { marker, text });
                s2 = &rest[text.len()..];
            } else {
                // hit the end
                output.push(Segment { marker, text: rest });
                break;
            }
        } else {
            // there's no marker at the beginning!
            if let Ok((_, (text, _))) = take_anything_until(parse_marker).parse(s2) {
                output.push(Segment {
                    marker: Marker {
                        take: text.len(),
                        repeat: 1,
                    },
                    text,
                });
                s2 = &s2[text.len()..];
            } else {
                output.push(Segment {
                    marker: Marker {
                        take: s2.len(),
                        repeat: 1,
                    },
                    text: s2,
                });
                break;
            }
        }
    }

    output
}

pub fn decompress(s: &str) -> String {
    let segments = segments(s);
    segments
        .into_iter()
        .map(|s| format!("{s}"))
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment() {
        let marker = Marker { take: 5, repeat: 3 };
        let segment = Segment {
            marker,
            text: "ABCDEfg",
        };
        assert_eq!(format!("{}", segment), "ABCDEABCDEABCDEfg")
    }

    #[test]
    fn test_segments() {
        assert_eq!(segments(""), vec![]);

        assert_eq!(
            segments("A"),
            vec![Segment {
                marker: Marker { take: 1, repeat: 1 },
                text: "A"
            }]
        );

        assert_eq!(
            segments("ABCDEFG"),
            vec![Segment {
                marker: Marker { take: 7, repeat: 1 },
                text: "ABCDEFG"
            }]
        );

        assert_eq!(
            segments("(1x5)A"),
            vec![Segment {
                marker: Marker { take: 1, repeat: 5 },
                text: "A"
            }]
        );

        assert_eq!(
            segments("(1x5)AB"),
            vec![Segment {
                marker: Marker { take: 1, repeat: 5 },
                text: "AB"
            }]
        );

        assert_eq!(
            segments("(1x5)AB(2x4)XYZ"),
            vec![
                Segment {
                    marker: Marker { take: 1, repeat: 5 },
                    text: "AB"
                },
                Segment {
                    marker: Marker { take: 2, repeat: 4 },
                    text: "XYZ"
                }
            ]
        );

        assert_eq!(
            segments("(10x9)AB(3x2)HIJ(10x13)LNWIKDMACM"),
            vec![
                Segment {
                    marker: Marker {
                        take: 10,
                        repeat: 9
                    },
                    text: "AB(3x2)HIJ"
                },
                Segment {
                    marker: Marker {
                        take: 10,
                        repeat: 13
                    },
                    text: "LNWIKDMACM"
                }
            ]
        );
    }

    #[test]
    fn test_decompress() {
        assert_eq!(decompress(""), "".to_string());
        assert_eq!(decompress("A"), "A".to_string());
        assert_eq!(decompress("(1x5)A"), "AAAAA".to_string());
        assert_eq!(decompress("(1x5)AB"), "AAAAAB".to_string());
        assert_eq!(decompress("(1x5)AB(2x4)XYZ"), "AAAAABXYXYXYXYZ".to_string());

        assert_eq!(decompress("ADVENT"), "ADVENT".to_string());
        assert_eq!(decompress("A(1x5)BC"), "ABBBBBC".to_string());
        assert_eq!(decompress("(3x3)XYZ"), "XYZXYZXYZ".to_string());
        assert_eq!(decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG".to_string());
        assert_eq!(decompress("(6x1)(1x3)A"), "(1x3)A".to_string());
        assert_eq!(
            decompress("X(8x2)(3x3)ABCY"),
            "X(3x3)ABC(3x3)ABCY".to_string()
        );
    }
}
