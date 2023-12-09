use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct HauntedWasteland(Vec<u8>, HashMap<Node, (Node, Node)>);

type Node = [u8; 3];

impl Fro for HauntedWasteland {
    fn fro(input: &str) -> Self {
        let (l, r) = input.split_once("\n\n").unwrap();

        let input = l.bytes().collect();

        let map = r
            .trim()
            .split('\n')
            .map(|line| {
                let (l, r) = line.split_once(" = ").unwrap();
                let a = l.bytes().collect::<Vec<_>>().try_into().unwrap();

                let (l, r) = r.split_once(", ").unwrap();
                let b = l
                    .trim_start_matches('(')
                    .bytes()
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                let c = r
                    .trim_end_matches(')')
                    .bytes()
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                (a, (b, c))
            })
            .collect::<HashMap<_, _>>();

        Self(input, map)
    }
}

impl HauntedWasteland {
    fn solve(&self, start: Node, go: impl Fn(Node) -> bool) -> i64 {
        self.0
            .iter()
            .cycle()
            .scan(start, |here, direction| {
                match direction {
                    b'L' => *here = self.1[here].0,
                    b'R' => *here = self.1[here].1,
                    _ => unreachable!(),
                }
                Some(*here)
            })
            .take_while(|node| go(*node))
            .count() as i64
            + 1
    }
}

impl Solution for HauntedWasteland {
    fn one(self) -> i64 {
        self.solve(*b"AAA", |node| node != *b"ZZZ")
    }

    fn two(self) -> i64 {
        self.1
            .keys()
            .filter(|node| node.ends_with(b"A"))
            .map(|start| self.solve(*start, |end| !end.ends_with(b"Z")))
            .fold(1, lcm)
    }
}
