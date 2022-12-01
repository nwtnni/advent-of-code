use aoc::*;

#[derive(Clone, Debug)]
pub struct CalorieCounting(Vec<Vec<i64>>);

impl Fro for CalorieCounting {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split("\n\n")
            .map(|lines| lines.split('\n').map(i64::fro).collect())
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for CalorieCounting {
    fn one(self) -> i64 {
        self.0
            .into_iter()
            .map(|calories| calories.into_iter().sum::<i64>())
            .max()
            .unwrap()
    }

    fn two(self) -> i64 {
        let mut sorted = self
            .0
            .into_iter()
            .map(|calories| calories.into_iter().sum::<i64>())
            .collect::<Vec<_>>();

        sorted.sort();
        sorted.into_iter().rev().take(3).sum()
    }
}
