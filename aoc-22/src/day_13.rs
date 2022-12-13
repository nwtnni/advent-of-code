use std::iter;

use aoc::*;

#[derive(Clone, Debug)]
pub struct DistressSignal(Vec<(Tree, Tree)>);

impl Fro for DistressSignal {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split("\n\n")
            .map(|line| {
                let (a, b) = line.split_once('\n').unwrap();
                let a = parse(&mut a.chars().peekable(), &mut String::new()).unwrap();
                let b = parse(&mut b.chars().peekable(), &mut String::new()).unwrap();
                (a, b)
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Tree {
    Leaf(i64),
    Node(Vec<Tree>),
}

fn parse(a: &mut iter::Peekable<impl Iterator<Item = char>>, buffer: &mut String) -> Option<Tree> {
    while let Some(c) = a.peek() {
        if *c == '[' {
            a.next();

            let mut node = Vec::new();
            while let Some(next) = parse(a, buffer) {
                node.push(next);
                if let Some(',') = a.peek() {
                    a.next();
                } else if let Some(']') = a.peek() {
                    break;
                }
            }

            a.next();
            return Some(Tree::Node(node));
        } else if *c == ']' {
            return None;
        } else if c.is_numeric() {
            while let Some(c) = a.peek() {
                if c.is_numeric() {
                    buffer.push(*c);
                    a.next();
                } else {
                    break;
                }
            }
            let b = Tree::Leaf(i64::fro(buffer));
            buffer.clear();
            return Some(b);
        } else {
            unreachable!()
        }
    }
    None
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

impl Eq for Tree {}
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
            .filter_map(|(i, (a, b))| if a < b { Some(i as i64 + 1) } else { None })
            .sum::<i64>()
    }

    fn two(self) -> i64 {
        let a = Tree::Node(vec![Tree::Node(vec![Tree::Leaf(2)])]);
        let b = Tree::Node(vec![Tree::Node(vec![Tree::Leaf(6)])]);

        let mut bs = self
            .0
            .into_iter()
            .flat_map(|(a, b)| [a, b])
            .chain(iter::once(a.clone()))
            .chain(iter::once(b.clone()))
            .collect::<Vec<_>>();
        bs.sort();

        let a = bs.iter().position(|x| *x == a).unwrap() as i64 + 1;
        let b = bs.iter().position(|x| *x == b).unwrap() as i64 + 1;
        a * b
    }
}
