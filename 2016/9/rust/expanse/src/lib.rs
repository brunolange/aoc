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
struct Node<'a> {
    marker: Marker,
    text: &'a str,
    children: Vec<Node<'a>>,
}

impl<'a> Display for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.marker)
    }
}

impl<'a> Node<'a> {
    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn print(&self, depth: usize) {
        println!("{}{self}", "  ".repeat(depth));
        for child in &self.children {
            child.print(depth + 1);
        }
    }
}

#[derive(Debug)]
struct Tree<'a>(Vec<Node<'a>>);

impl<'a> Tree<'a> {
    fn new(nodes: Vec<Node<'a>>) -> Self {
        Tree(nodes)
    }

    #[allow(unused)]
    fn print(&self) {
        for node in &self.0 {
            node.print(0);
        }
    }

    fn assemble(s: &'a str, curr_depth: usize, max_depth: Option<usize>) -> Self {
        let mut nodes = Vec::new();
        if curr_depth >= max_depth.unwrap_or(curr_depth + 1) {
            return Self::new(nodes);
        }

        let mut s = s;
        while !s.is_empty() {
            if let Ok((tail, marker)) = parse_marker.parse(s) {
                let take = marker.take;
                let text = &tail[..take];
                let subtree = Self::assemble(text, curr_depth + 1, max_depth);
                nodes.push(Node {
                    marker,
                    text,
                    children: subtree.0,
                });
                s = &tail[take..];
            } else {
                // push leaf nodes to wrap single character
                nodes.push(Node {
                    marker: Marker { take: 1, repeat: 1 },
                    text: &s[..1],
                    children: vec![],
                });
                s = &s[1..];
            }
        }
        Self::new(nodes)
    }

    fn build(s: &'a str) -> Self {
        Self::assemble(s, 0, None)
    }

    fn build_up_to(s: &'a str, depth: usize) -> Self {
        Self::assemble(s, 0, Some(depth))
    }

    fn count(&self) -> usize {
        fn _count(nodes: &[Node]) -> usize {
            nodes
                .iter()
                .map(|node| {
                    if node.is_leaf() {
                        node.marker.repeat * node.marker.take
                    } else {
                        node.marker.repeat * _count(&node.children)
                    }
                })
                .sum()
        }
        _count(&self.0)
    }

    fn decompress(&self) -> String {
        fn _decompress(nodes: &[Node]) -> String {
            nodes
                .iter()
                .map(|node| {
                    if node.is_leaf() {
                        node.text.repeat(node.marker.repeat)
                    } else {
                        _decompress(&node.children).repeat(node.marker.repeat)
                    }
                })
                .collect::<Vec<_>>()
                .join("")
        }
        _decompress(&self.0)
    }
}

pub fn decoded_count(s: &str) -> usize {
    Tree::build(s).count()
}

pub fn decoded_count_up_to(s: &str, max_depth: usize) -> usize {
    Tree::build_up_to(s, max_depth).count()
}

pub fn decompress(s: &str) -> String {
    let tree = Tree::build(s);
    tree.decompress()
}

pub fn decompress_up_to(s: &str, max_depth: usize) -> String {
    let tree = Tree::build_up_to(s, max_depth);
    tree.decompress()
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
        assert_eq!(decompress_up_to("(6x1)(1x3)A", 1), "(1x3)A".to_string());
        assert_eq!(decompress_up_to("(6x1)(1x3)A", 2), "AAA".to_string());
        assert_eq!(decompress_up_to("(6x1)(1x3)A", 3), "AAA".to_string());
        assert_eq!(
            decompress_up_to("X(8x2)(3x3)ABCY", 1),
            "X(3x3)ABC(3x3)ABCY".to_string()
        );
        assert_eq!(
            decompress_up_to("X(8x2)(3x3)ABCY", 100),
            "XABCABCABCABCABCABCY".to_string()
        );

        assert_eq!(
            decompress("A(2x2)BCdef(2x2)XYzzzzzzz"),
            "ABCBCdefXYXYzzzzzzz".to_string()
        );
    }

    #[test]
    fn test_count() {
        assert_eq!(decoded_count(""), "".len());
        assert_eq!(decoded_count("A"), "A".len());
        assert_eq!(decoded_count("(1x5)A"), "AAAAA".len());
        assert_eq!(decoded_count("(1x5)AB"), "AAAAAB".len());
        assert_eq!(decoded_count("(1x5)AB(2x4)XYZ"), "AAAAABXYXYXYXYZ".len());

        assert_eq!(decoded_count("ADVENT"), "ADVENT".len());
        assert_eq!(decoded_count("A(1x5)BC"), "ABBBBBC".len());
        assert_eq!(decoded_count("(3x3)XYZ"), "XYZXYZXYZ".len());
        assert_eq!(decoded_count("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG".len());
        assert_eq!(decoded_count_up_to("(6x1)(1x3)A", 1), "(1x3)A".len());
        assert_eq!(decoded_count_up_to("(6x1)(1x3)A", 2), "AAA".len());
        assert_eq!(decoded_count_up_to("(6x1)(1x3)A", 3), "AAA".len());
        assert_eq!(
            decoded_count_up_to("X(8x2)(3x3)ABCY", 1),
            "X(3x3)ABC(3x3)ABCY".len()
        );
        assert_eq!(
            decoded_count_up_to("X(8x2)(3x3)ABCY", 100),
            "XABCABCABCABCABCABCY".len()
        );

        assert_eq!(
            decoded_count("A(2x2)BCdef(2x2)XYzzzzzzz"),
            "ABCBCdefXYXYzzzzzzz".len()
        );
    }
}
