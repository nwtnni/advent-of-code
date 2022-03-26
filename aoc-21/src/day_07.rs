use std::cmp;

use aoc::*;

#[derive(Clone, Debug)]
pub struct TheTreacheryOfWhales(Vec<i64>);

impl Fro for TheTreacheryOfWhales {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split(',')
            .map(i64::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for TheTreacheryOfWhales {
    fn one(self) -> i64 {
        let (min, max) = self
            .0
            .iter()
            .fold((i64::MAX, i64::MIN), |(min, max), crab| {
                (cmp::min(min, *crab), cmp::max(max, *crab))
            });

        (min..max)
            .map(|pos| {
                self.0
                    .iter()
                    .map(move |crab| (pos - crab).abs())
                    .sum::<i64>()
            })
            .min()
            .unwrap()
    }

    fn two(self) -> i64 {
        let (min, max) = self
            .0
            .iter()
            .fold((i64::MAX, i64::MIN), |(min, max), crab| {
                (cmp::min(min, *crab), cmp::max(max, *crab))
            });

        let cost = |distance: i64| (distance * (distance + 1)) / 2;

        (min..max)
            .map(|pos| {
                self.0
                    .iter()
                    .map(|crab| cost((pos - crab).abs()))
                    .sum::<i64>()
            })
            .min()
            .unwrap()
    }
}
