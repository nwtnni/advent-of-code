#![allow(clippy::identity_op)]

use aoc::*;

#[derive(Clone, Debug)]
pub struct RockPaperScissors(Vec<(char, char)>);

impl Fro for RockPaperScissors {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let mut iter = line.chars();
                let a = iter.give();
                iter.next();
                let b = iter.give();
                (a, b)
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for RockPaperScissors {
    fn one(self) -> i64 {
        self.0
            .into_iter()
            .map(|(a, b)| match (a, b) {
                ('A', 'X') => 1 + 3,
                ('B', 'X') => 1 + 0,
                ('C', 'X') => 1 + 6,

                ('A', 'Y') => 2 + 6,
                ('B', 'Y') => 2 + 3,
                ('C', 'Y') => 2 + 0,

                ('A', 'Z') => 3 + 0,
                ('B', 'Z') => 3 + 6,
                ('C', 'Z') => 3 + 3,

                _ => unreachable!(),
            })
            .sum()
    }

    fn two(self) -> i64 {
        self.0
            .into_iter()
            .map(|(a, b)| match (a, b) {
                ('A', 'X') => 3 + 0,
                ('B', 'X') => 1 + 0,
                ('C', 'X') => 2 + 0,

                ('A', 'Y') => 1 + 3,
                ('B', 'Y') => 2 + 3,
                ('C', 'Y') => 3 + 3,

                ('A', 'Z') => 2 + 6,
                ('B', 'Z') => 3 + 6,
                ('C', 'Z') => 1 + 6,

                _ => unreachable!(),
            })
            .sum()
    }
}
