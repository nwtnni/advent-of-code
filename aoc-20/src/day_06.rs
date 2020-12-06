use std::collections::HashMap;
use std::collections::HashSet;

use aoc::*;

pub struct CustomCustoms(Vec<String>);

impl Fro for CustomCustoms {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split("\n\n")
            .map(String::from)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for CustomCustoms {
    fn one(self) -> i64 {
        let mut count = 0;
        for group in self.0 {
            let mut set = HashSet::new();
            for people in group.split_whitespace() {
                for answer in people.chars() {
                    set.insert(answer);
                }
            }
            count += set.len();
        }
        count as i64
    }

    fn two(self) -> i64 {
        let mut total = 0;
        for group in self.0 {
            let mut set = HashMap::new();
            let mut peoples = 0;
            for people in group.split_whitespace() {
                peoples += 1;
                for answer in people.chars() {
                    set.entry(answer)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }

            for (_, count) in set {
                if count == peoples {
                    total += 1;
                }
            }
        }

        total as i64
    }
}
