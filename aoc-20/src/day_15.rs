use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct RambunctiousRecitation(Vec<i64>);

impl Fro for RambunctiousRecitation {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split(',')
            .map(i64::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl RambunctiousRecitation {
    fn speak(&self, to: usize) -> i64 {
        let mut spoken = self
            .0
            .iter()
            .enumerate()
            .map(|(i, n)| (*n, i))
            .take(self.0.len() - 1)
            .collect::<HashMap<_, _>>();

        let mut last = self.0.last().copied().unwrap();

        for i in self.0.len() - 1..to - 1 {
            let save = last;

            last = spoken
                .get(&last)
                .copied()
                .map(|j| i as i64 - j as i64)
                .unwrap_or(0);

            spoken.insert(save, i);
        }

        last
    }
}

impl Solution for RambunctiousRecitation {
    fn one(self) -> i64 {
        self.speak(2020)
    }

    fn two(self) -> i64 {
        self.speak(30000000)
    }
}
