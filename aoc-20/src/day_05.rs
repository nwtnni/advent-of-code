use std::collections::BTreeSet;
use std::cmp;

use aoc::*;

pub struct BinaryBoarding(Vec<Vec<u8>>);

impl Fro for BinaryBoarding {
    fn fro(input: &str) -> Self {
        input.trim()
            .split('\n')
            .map(|line| Vec::from(line.as_bytes()))
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for BinaryBoarding {
    fn one(self) -> i64 {
        self.0
            .into_iter()
            .map(|pass| {
                let mut bot = 0;
                let mut top = 128;

                let mut left = 0;
                let mut right = 8;

                for byte in pass {
                    let vertical = (bot + top) / 2;
                    let horizontal = (left + right) / 2;
                    match byte {
                    | b'F' => top = vertical,
                    | b'B' => bot = vertical,
                    | b'R' => left = horizontal,
                    | b'L' => right = horizontal,
                    | _ => unreachable!(),
                    }
                }

                bot * 8 + left
            })
            .fold(0, cmp::max)
    }

    fn two(self) -> i64 {
        let mut seats = self.0
            .into_iter()
            .map(|pass| {
                let mut bot = 0;
                let mut top = 128;

                let mut left = 0;
                let mut right = 8;

                for byte in pass {
                    let vertical = (bot + top) / 2;
                    let horizontal = (left + right) / 2;
                    match byte {
                    | b'F' => top = vertical,
                    | b'B' => bot = vertical,
                    | b'R' => left = horizontal,
                    | b'L' => right = horizontal,
                    | _ => unreachable!(),
                    }
                }

                bot * 8 + left
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .peekable();

        while let (Some(lo), Some(hi)) = (seats.next(), seats.peek()) {
            if lo + 2 == *hi {
                return lo + 1;
            }
        }

        unreachable!()
    }
}
