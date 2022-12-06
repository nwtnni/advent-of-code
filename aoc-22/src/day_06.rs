use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct TuningTrouble(String);

impl Fro for TuningTrouble {
    fn fro(input: &str) -> Self {
        Self(String::from(input.trim()))
    }
}

impl Solution for TuningTrouble {
    fn one(self) -> i64 {
        self.start(4) as i64
    }

    fn two(self) -> i64 {
        self.start(14) as i64
    }
}

impl TuningTrouble {
    fn start(&self, length: usize) -> usize {
        let mut set = HashSet::new();

        self.0
            .as_bytes()
            .windows(length)
            .position(|message| {
                set.clear();
                set.extend(message.iter().copied());
                set.len() == length
            })
            .unwrap()
            + length
    }
}
