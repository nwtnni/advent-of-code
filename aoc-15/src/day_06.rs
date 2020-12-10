use std::cmp;

use aoc::*;

#[derive(Clone, Debug)]
pub struct ProbablyAFireHazard(Vec<(Op, Pos, Pos)>);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Op {
    On,
    Off,
    Toggle,
}

impl Fro for ProbablyAFireHazard {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let op = if line.starts_with("turn on") {
                    Op::On
                } else if line.starts_with("turn off") {
                    Op::Off
                } else if line.starts_with("toggle") {
                    Op::Toggle
                } else {
                    unreachable!()
                };

                let mut iter = line
                    .trim()
                    .trim_start_matches("turn on ")
                    .trim_start_matches("turn off ")
                    .trim_start_matches("toggle ")
                    .split(" through ");

                let mut lo = iter.give().split(',').map(i64::fro);
                let mut hi = iter.give().split(',').map(i64::fro);

                (
                    op,
                    Pos { x: lo.give(), y: lo.give() },
                    Pos { x: hi.give(), y: hi.give() },
                )
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for ProbablyAFireHazard {
    fn one(self) -> i64 {
        let mut grid = vec![vec![false; 1000]; 1000];

        for (op, lo, hi) in self.0 {
            for Pos { x, y } in lo.to_inclusive(hi) {
                grid[y as usize][x as usize] = match op {
                | Op::On => true,
                | Op::Off => false,
                | Op::Toggle => !grid[y as usize][x as usize],
                }
            }
        }

        grid.into_iter()
            .flatten()
            .filter(|on| *on)
            .count()
            as i64
    }

    fn two(self) -> i64 {
        let mut grid = vec![vec![0i64; 1000]; 1000];

        for (op, lo, hi) in self.0 {
            for Pos { x, y } in lo.to_inclusive(hi) {
                let brightness = &mut grid[y as usize][x as usize];
                *brightness = match op {
                | Op::On => *brightness + 1,
                | Op::Off => cmp::max(0, *brightness - 1),
                | Op::Toggle => *brightness + 2,
                };
            }
        }

        grid.into_iter()
            .flatten()
            .sum()
    }
}
