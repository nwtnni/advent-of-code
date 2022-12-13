use std::iter;

use aoc::*;

#[derive(Clone, Debug)]
pub struct DistressSignal(Vec<[Tree; 2]>);

impl Fro for DistressSignal {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split("\n\n")
            .map(|line| {
                let mut buffer = String::new();
                let (a, b) = line.split_once('\n').unwrap();
                let a = parse(&mut a.chars().peekable(), &mut buffer).unwrap();
                let b = parse(&mut b.chars().peekable(), &mut buffer).unwrap();
                [a, b]
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tree {
    Leaf(i64),
    Node(Vec<Tree>),
}

fn parse(
    iter: &mut iter::Peekable<impl Iterator<Item = char>>,
    buffer: &mut String,
) -> Option<Tree> {
    match iter.peek()? {
        '[' => {
            iter.next();

            let mut node = Vec::new();
            while let Some(next) = parse(iter, buffer) {
                node.push(next);
                if let Some(',') = iter.peek() {
                    iter.next();
                } else if let Some(']') = iter.peek() {
                    break;
                }
            }

            iter.next();
            Some(Tree::Node(node))
        }
        ']' => None,
        char if char.is_numeric() => {
            while let Some(char) = iter.peek() {
                if char.is_numeric() {
                    buffer.push(*char);
                    iter.next();
                } else {
                    break;
                }
            }

            let leaf = Tree::Leaf(i64::fro(buffer));
            buffer.clear();
            Some(leaf)
        }
        _ => unreachable!(),
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Tree::Leaf(a), Tree::Leaf(b)) => a.cmp(b),
            (Tree::Node(a), Tree::Node(b)) => a.cmp(b),
            (Tree::Leaf(a), b @ Tree::Node(_)) => (Tree::Node(vec![Tree::Leaf(*a)])).cmp(b),
            (a @ Tree::Node(_), Tree::Leaf(b)) => a.cmp(&Tree::Node(vec![Tree::Leaf(*b)])),
        }
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution for DistressSignal {
    fn one(self) -> i64 {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, [a, b])| if a < b { Some(i as i64 + 1) } else { None })
            .sum::<i64>()
    }

    fn two(self) -> i64 {
        let keys = [
            Tree::Node(vec![Tree::Node(vec![Tree::Leaf(2)])]),
            Tree::Node(vec![Tree::Node(vec![Tree::Leaf(6)])]),
        ];

        let mut sorted = self
            .0
            .into_iter()
            .flatten()
            .chain(keys.clone())
            .collect::<Vec<_>>();

        sorted.sort();

        keys.iter()
            .map(|key| sorted.iter().position(|packet| packet == key).unwrap() as i64 + 1)
            .product()
    }
}
