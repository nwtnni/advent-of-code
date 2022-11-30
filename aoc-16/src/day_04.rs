use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct SecurityThroughObscurity(Vec<Room>);

#[derive(Clone, Debug)]
struct Room {
    name: String,
    id: i64,
    checksum: [char; 5],
}

impl Fro for SecurityThroughObscurity {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (name, metadata) = line.rsplit_once('-').unwrap();
                let (id, checksum) = metadata.trim_end_matches(']').split_once('[').unwrap();
                let mut iter = checksum.chars();
                Room {
                    name: String::from(name),
                    id: i64::fro(id),
                    checksum: [
                        iter.give(),
                        iter.give(),
                        iter.give(),
                        iter.give(),
                        iter.give(),
                    ],
                }
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for SecurityThroughObscurity {
    fn one(self) -> i64 {
        self.0
            .iter()
            .filter(|room| {
                let mut counts = HashMap::new();

                room.name
                    .chars()
                    .filter(|char| *char != '-')
                    .for_each(|char| {
                        *counts.entry(char).or_insert(0) += 1;
                    });

                for window in room.checksum.windows(2) {
                    let a = counts.get(&window[0]).copied().unwrap_or(0);
                    let b = counts.get(&window[1]).copied().unwrap_or(0);

                    if a < b || a == b && window[0] > window[1] {
                        return false;
                    }
                }

                true
            })
            .map(|room| room.id)
            .sum::<i64>()
    }

    fn two(self) -> i64 {
        todo!()
    }
}
