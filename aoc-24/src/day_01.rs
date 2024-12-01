use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct HistorianHysteria(Vec<(i64, i64)>);

impl Fro for HistorianHysteria {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let mut iter = line.split_whitespace();
                let l = iter.give().to();
                let r = iter.give().to();
                (l, r)
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for HistorianHysteria {
    fn one(self) -> i64 {
        let (mut ls, mut rs) = self.0.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();
        ls.sort();
        rs.sort();
        ls.into_iter()
            .zip(rs)
            .map(|(l, r)| l.abs_diff(r))
            .sum::<u64>() as i64
    }

    fn two(self) -> i64 {
        let mut counts = HashMap::new();

        self.0
            .iter()
            .for_each(|(_, r)| *counts.entry(r).or_insert(0) += 1);

        self.0
            .iter()
            .map(|(l, _)| l * counts.get(l).copied().unwrap_or(0))
            .sum()
    }
}
