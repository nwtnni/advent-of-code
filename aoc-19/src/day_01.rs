use std::str;

use aoc::*;

pub struct TheTyrannyOfTheRocketEquation(Vec<i64>);

impl Fro for TheTyrannyOfTheRocketEquation {
    fn fro(input: &str) -> Self {
        input
            .split_whitespace()
            .map(i64::fro)
            .collect::<Vec<_>>()
            .tap(TheTyrannyOfTheRocketEquation)
    }
}

impl Solution for TheTyrannyOfTheRocketEquation {
    fn one(self) -> i64 {
        self.0.iter().map(|mass| mass / 3 - 2).sum()
    }

    fn two(self) -> i64 {
        self.0
            .iter()
            .map(|mass| {
                let mut fuel = mass / 3 - 2;
                let mut df = fuel / 3 - 2;
                while df > 0 {
                    fuel += df;
                    df = df / 3 - 2;
                }
                fuel
            })
            .sum()
    }
}
