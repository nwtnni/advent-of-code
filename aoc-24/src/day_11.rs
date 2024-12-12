use std::mem;

use aoc::*;

#[derive(Clone, Debug)]
pub struct PlutonianPebbles(Vec<i64>);

impl Fro for PlutonianPebbles {
    fn fro(input: &str) -> Self {
        input
            .split_whitespace()
            .map(i64::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for PlutonianPebbles {
    fn one(self) -> i64 {
        let mut prev = Vec::new();
        let mut next = Vec::new();

        self.0
            .iter()
            .map(|stone| {
                prev.clear();
                next.clear();

                prev.push(*stone);

                for _ in 0..25 {
                    next.clear();
                    for stone in &prev {
                        blink(*stone, &mut next);
                    }
                    mem::swap(&mut prev, &mut next);
                }

                prev.len() as i64
            })
            .sum()
    }

    fn two(self) -> i64 {
        let mut prev = Counter::default();
        let mut next = Counter::default();

        self.0.into_iter().for_each(|stone| prev.update_one(stone));

        for _ in 0..75 {
            next.clear();
            for (stone, count) in prev.iter() {
                blink_map(*stone, &mut next, *count);
            }
            mem::swap(&mut prev, &mut next);
        }

        prev.values().copied().sum()
    }
}

fn blink(stone: i64, next: &mut Vec<i64>) {
    if stone == 0 {
        next.push(1);
        return;
    }

    let digits = stone.ilog10() + 1;
    if digits & 1 == 0 {
        next.push(stone / 10i64.pow(digits >> 1));
        next.push(stone % 10i64.pow(digits >> 1));
    } else {
        next.push(stone * 2024);
    }
}

fn blink_map(stone: i64, next: &mut Counter<i64>, count: i64) {
    if stone == 0 {
        next.update(1, count);
        return;
    }

    let digits = stone.ilog10() + 1;
    if digits & 1 == 0 {
        next.update(stone / 10i64.pow(digits >> 1), count);
        next.update(stone % 10i64.pow(digits >> 1), count);
    } else {
        next.update(stone * 2024, count);
    }
}
