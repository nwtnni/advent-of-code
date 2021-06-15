use std::cmp;
use std::iter;

use petgraph::graphmap::DiGraphMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct KnightsOfTheDinnerTable(DiGraphMap<&'static str, i64>);

impl Fro for KnightsOfTheDinnerTable {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .filter_map(|line| {
                let line = line.strip_suffix('.')?;
                let (left, right) = line.split_once(" happiness units by sitting next to ")?;
                let (left, value) = if let Some((left, value)) = left.split_once(" would gain ") {
                    (left, i64::fro(value))
                } else if let Some((left, value)) = left.split_once(" would lose ") {
                    (left, -i64::fro(value))
                } else {
                    unreachable!()
                };
                Some((left.leak(), right.leak(), value))
            })
            .collect::<DiGraphMap<_, _>>()
            .tap(Self)
    }
}

impl Solution for KnightsOfTheDinnerTable {
    fn one(self) -> i64 {
        let mut permutation = self.0.nodes().collect::<Vec<_>>();
        let mut max = i64::MIN;
        let len = permutation.len();

        for (i, j) in permute(len as u8) {
            permutation.swap(i, j);

            let total = permutation
                .windows(2)
                .chain(iter::once([permutation[len - 1], permutation[0]].as_ref()))
                .map(|window| self.0[(window[0], window[1])] + self.0[(window[1], window[0])])
                .sum::<i64>();

            max = cmp::max(total, max);
        }

        max
    }

    fn two(mut self) -> i64 {
        self.0
            .nodes()
            .collect::<Vec<_>>()
            .into_iter()
            .flat_map(|node| iter::once(("I", node, 0)).chain(iter::once((node, "I", 0))))
            .tap(|edges| self.0.extend(edges));
        self.one()
    }
}
