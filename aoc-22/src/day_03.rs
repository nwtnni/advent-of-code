use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct RucksackReorganization(Vec<Vec<char>>);

impl Fro for RucksackReorganization {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| line.chars().collect())
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for RucksackReorganization {
    fn one(self) -> i64 {
        self.0
            .into_iter()
            .map(|sack| {
                let len = sack.len();
                let half = sack[0..len / 2].iter().copied().collect::<HashSet<_>>();
                sack[len / 2..]
                    .iter()
                    .copied()
                    .find(|item| half.contains(item))
                    .unwrap()
            })
            .map(priority)
            .sum()
    }

    fn two(self) -> i64 {
        self.0
            .chunks(3)
            .map(|group| {
                let first = group[0].iter().copied().collect::<HashSet<_>>();

                let second = group[1]
                    .iter()
                    .copied()
                    .filter(|item| first.contains(item))
                    .collect::<HashSet<_>>();

                group[2]
                    .iter()
                    .copied()
                    .find(|item| second.contains(item))
                    .unwrap()
            })
            .map(priority)
            .sum()
    }
}

fn priority(item: char) -> i64 {
    match item.is_lowercase() {
        true => item as u8 as i64 - b'a' as i64 + 1,
        false => item as u8 as i64 - b'A' as i64 + 27,
    }
}
