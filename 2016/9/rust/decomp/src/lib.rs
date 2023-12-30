mod parsers;

use std::{fmt::Display, str::FromStr};

use nom::Parser;
use parsers::parse_marker;

#[derive(Debug, PartialEq)]
pub struct Marker {
    take: usize,
    repeat: usize,
}

impl FromStr for Marker {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, marker) = parse_marker(s).map_err(|_| ())?;

        Ok(marker)
    }
}

#[derive(Debug)]
struct Node {
    marker: Marker,
    children: Vec<Node>,
}

impl Node {
    fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    fn print(&self, depth: usize) {
        println!("{}{self}", "  ".repeat(depth));
        for child in &self.children {
            child.print(depth + 1);
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self.marker))
    }
}

fn tree(s: &str, curr_depth: usize, max_depth: Option<usize>) -> Vec<Node> {
    let mut output = Vec::new();
    if curr_depth >= max_depth.unwrap_or(curr_depth + 1) {
        return output;
    }

    let mut s2 = s;
    loop {
        if let Ok((tail, marker)) = parse_marker.parse(s2) {
            let index = marker.take;
            output.push(Node {
                marker,
                children: tree(&tail[..index], curr_depth + 1, max_depth),
            });
            s2 = &tail[index..];
        } else {
            break;
        }
    }
    output
}

pub fn decoded_count(s: &str) -> usize {
    let tree = tree(s, 0, None);
    count(&tree)
}

pub fn decoded_count_up_to(s: &str, max_depth: usize) -> usize {
    let tree = tree(s, 0, Some(max_depth));
    count(&tree)
}

pub fn decompress(s: &str) -> String {
    todo!()
}

fn count(tree: &Vec<Node>) -> usize {
    tree.into_iter()
        .map(|marker_node| {
            if marker_node.is_leaf() {
                marker_node.marker.repeat * marker_node.marker.take
            } else {
                marker_node.marker.repeat * count(&marker_node.children)
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_build_tree() {
        let s = "(6x9)JUORKH(10x13)LNWIKDMACM(126x14)(21x8)QLKUJNVVZIQGGFCJZMPHK(2x1)ZH(59x3)(38x14)KELEPIDYLCGJUBCXACRSOCEZYXLOFJSADZAYXN(8x11)HORSWAQU(21x2)YEZNNYDLDSTGWMQFSMTEZ";

        let tree = tree(s, 0, None);
        for marker_node in &tree {
            marker_node.print(0);
        }

        println!("count = {:?}", count(&tree));
    }
}
