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
        write!(f, "{}", format!("{:?}", self.marker))
    }
}

impl<'a> Node<'a> {
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

        let mut s2 = s;
        loop {
            if let Ok((tail, marker)) = parse_marker.parse(s2) {
                let take = marker.take;
                let text = &tail[..take];
                let subtree = Self::assemble(text, curr_depth + 1, max_depth);
                nodes.push(Node {
                    marker,
                    text,
                    children: subtree.0,
                });
                s2 = &tail[take..];
            } else {
                break;
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
        fn _count(nodes: &Vec<Node>) -> usize {
            nodes
                .into_iter()
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
        fn _decompress(nodes: &Vec<Node>) -> String {
            nodes
                .into_iter()
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
        // assert_eq!(decompress(""), "".to_string());
        // assert_eq!(decompress("A"), "A".to_string());
        // assert_eq!(decompress("(1x5)A"), "AAAAA".to_string());
        // assert_eq!(decompress("(1x5)AB"), "AAAAAB".to_string());
        // assert_eq!(decompress("(1x5)AB(2x4)XYZ"), "AAAAABXYXYXYXYZ".to_string());

        // assert_eq!(decompress("ADVENT"), "ADVENT".to_string());
        // assert_eq!(decompress("A(1x5)BC"), "ABBBBBC".to_string());
        // assert_eq!(decompress("(3x3)XYZ"), "XYZXYZXYZ".to_string());
        // assert_eq!(decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG".to_string());
        // assert_eq!(decompress("(6x1)(1x3)A"), "(1x3)A".to_string());
        // assert_eq!(
        //     decompress("X(8x2)(3x3)ABCY"),
        //     "X(3x3)ABC(3x3)ABCY".to_string()
        // );
    }

    #[test]
    fn test_tree() {
        // let s = "(126x14)(21x8)QLKUJNVVZIQGGFCJZMPHK(2x1)ZH(59x3)(38x14)KELEPIDYLCGJUBCXACRSOCEZYXLOFJSADZAYXN(8x11)HORSWAQU(21x2)YEZNNYDLDSTGWMQFSMTEZ";
        // let s = "ADVENT";
        let s = "(7x2)(2x3)AB";

        let tree = Tree::build(s);
        tree.print();

        // println!("count = {:?}", tree.count());
        println!("decompressed = {}", decompress(s));
    }
}
