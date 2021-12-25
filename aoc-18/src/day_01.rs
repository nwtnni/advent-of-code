use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct ChronalCalibration(Vec<i64>);

impl Fro for ChronalCalibration {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(i64::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for ChronalCalibration {
    fn one(self) -> i64 {
        self.0.iter().sum()
    }

    fn two(self) -> i64 {
        let mut seen = HashSet::new();

        self.0
            .iter()
            .cycle()
            .scan(0, |frequency, change| {
                *frequency += change;
                Some(*frequency)
            })
            .find_map(|frequency| match seen.insert(frequency) {
                false => Some(frequency),
                true => None,
            })
            .unwrap()
    }
}
