use std::cmp;

use petgraph::graphmap::UnGraphMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct AllInASingleNight(UnGraphMap<&'static str, i64>);

impl Fro for AllInASingleNight {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .filter_map(|line| {
                let (pair, distance) = line.split_once(" = ")?;
                let (from, to) = pair.split_once(" to ")?;
                Some((from.leak(), to.leak(), i64::fro(distance)))
            })
            .collect::<UnGraphMap<_, _>>()
            .tap(Self)
    }
}

impl AllInASingleNight {
    fn fold(&self, init: i64, apply: fn(i64, i64) -> i64) -> i64 {
        let mut permutation = self.0.nodes().collect::<Vec<_>>();
        let mut accumulator = init;

        for (i, j) in permute(permutation.len() as u8) {
            permutation.swap(i, j);

            let distance = permutation
                .windows(2)
                .map(|window| self.0[(window[0], window[1])])
                .sum::<i64>();

            accumulator = apply(distance, accumulator);
        }

        accumulator
    }
}

impl Solution for AllInASingleNight {
    fn one(self) -> i64 {
        self.fold(i64::MAX, cmp::min)
    }

    fn two(self) -> i64 {
        self.fold(i64::MIN, cmp::max)
    }
}
