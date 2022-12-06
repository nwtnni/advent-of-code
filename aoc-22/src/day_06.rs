use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct TuningTrouble(Vec<char>);

impl Fro for TuningTrouble {
    fn fro(input: &str) -> Self {
        Self(input.chars().collect())
    }
}

impl Solution for TuningTrouble {
    fn one(self) -> i64 {
        for (i, a) in self.0.windows(4).enumerate() {
            if a[0] != a[1]
                && a[1] != a[2]
                && a[2] != a[3]
                && a[0] != a[2]
                && a[0] != a[3]
                && a[1] != a[3]
            {
                return 4 + i as i64;
            }
        }
        panic!()
    }

    fn two(self) -> i64 {
        for (i, a) in self.0.windows(14).enumerate() {
            if a.iter().collect::<HashSet<_>>().len() == 14 {
                return 14 + i as i64;
            }
        }
        panic!()
    }
}
