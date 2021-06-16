use std::convert::TryFrom as _;
use std::iter;
use std::mem;

use aoc::*;

#[derive(Clone, Debug)]
pub struct LikeAGIFForYourYard([u128; 102]);

impl Fro for LikeAGIFForYourYard {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let mut mask = 0b1;
                let mut row = 0u128;
                for bit in line.chars().rev() {
                    mask <<= 1;
                    match bit {
                        '#' => row |= mask,
                        '.' => (),
                        _ => unreachable!(),
                    }
                }
                row
            })
            .tap(|grid| iter::once(0u128).chain(grid))
            .chain(iter::once(0u128))
            .collect::<Vec<_>>()
            .tap(<[u128; 102]>::try_from)
            .unwrap()
            .tap(Self)
    }
}

impl Solution for LikeAGIFForYourYard {
    fn one(self) -> i64 {
        let mut prev = [0u128; 102];
        let mut next = self.0;

        for _ in 0..100 {
            mem::swap(&mut prev, &mut next);

            for row in 1..101 {
                let mut top = 0b111;
                let mut mid = 0b101;
                let mut bot = 0b111;

                let mut sel = 0b010;

                for _ in 1..101 {
                    let light = (prev[row] & sel) > 0;
                    let neighbors = (prev[row - 1] & top).count_ones()
                        + (prev[row] & mid).count_ones()
                        + (prev[row + 1] & bot).count_ones();

                    match (light, neighbors) {
                        (true, 2..=3) => next[row] |= sel,
                        (true, _) => next[row] &= !sel,
                        (false, 3) => next[row] |= sel,
                        (false, _) => next[row] &= !sel,
                    }

                    top <<= 1;
                    mid <<= 1;
                    bot <<= 1;
                    sel <<= 1;
                }
            }
        }

        next.iter().map(|row| row.count_ones()).sum::<u32>() as i64
    }

    fn two(self) -> i64 {
        let mut prev = [0u128; 102];
        let mut next = self.0;

        next[1] |= 1 << 1;
        next[1] |= 1 << 100;
        next[100] |= 1 << 1;
        next[100] |= 1 << 100;

        for _ in 0..100 {
            mem::swap(&mut prev, &mut next);

            for row in 1..101 {
                let mut top = 0b111;
                let mut mid = 0b101;
                let mut bot = 0b111;

                let mut sel = 0b010;

                for _ in 1..101 {
                    let light = (prev[row] & sel) > 0;
                    let neighbors = (prev[row - 1] & top).count_ones()
                        + (prev[row] & mid).count_ones()
                        + (prev[row + 1] & bot).count_ones();

                    match (light, neighbors) {
                        (true, 2..=3) => next[row] |= sel,
                        (true, _) => next[row] &= !sel,
                        (false, 3) => next[row] |= sel,
                        (false, _) => next[row] &= !sel,
                    }

                    top <<= 1;
                    mid <<= 1;
                    bot <<= 1;
                    sel <<= 1;
                }
            }

            next[1] |= 1 << 1;
            next[1] |= 1 << 100;
            next[100] |= 1 << 1;
            next[100] |= 1 << 100;
        }

        next.iter().map(|row| row.count_ones()).sum::<u32>() as i64
    }
}
