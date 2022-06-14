use std::cmp;

use aoc::*;

#[derive(Clone, Debug)]
pub struct BathroomSecurity(Vec<Vec<Dir>>);

impl Fro for BathroomSecurity {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                line.chars()
                    .map(|char| match char {
                        'U' => Dir::N,
                        'D' => Dir::S,
                        'L' => Dir::W,
                        'R' => Dir::E,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for BathroomSecurity {
    fn one(self) -> i64 {
        let mut total = 0;

        for (place, directions) in self.0.into_iter().rev().enumerate() {
            let (x, y) = directions
                .into_iter()
                .fold((0usize, 0usize), |(x, y), direction| match direction {
                    Dir::N => (x, cmp::min(2, y.saturating_sub(1))),
                    Dir::S => (x, cmp::min(2, y + 1)),
                    Dir::W => (cmp::min(2, x.saturating_sub(1)), y),
                    Dir::E => (cmp::min(2, x + 1), y),
                });

            total += 10i64.pow(place as u32) * [[1, 2, 3], [4, 5, 6], [7, 8, 9]][y][x]
        }

        total
    }

    fn two(self) -> i64 {
        for directions in self.0 {
            fn clamp(keep: i64, change: i64, delta: i64) -> i64 {
                if keep.abs() + (change + delta).abs() > 2 {
                    change
                } else {
                    change + delta
                }
            }

            let (x, y) = directions
                .into_iter()
                .fold((0i64, 0i64), |(x, y), direction| match direction {
                    Dir::N => (x, clamp(x, y, -1)),
                    Dir::S => (x, clamp(x, y, 1)),
                    Dir::W => (clamp(y, x, -1), y),
                    Dir::E => (clamp(y, x, 1), y),
                });

            let x = usize::try_from(x + 2).unwrap();
            let y = usize::try_from(y + 2).unwrap();

            print!(
                "{}",
                [
                    ['_', '_', '1', '_', '_'],
                    ['_', '2', '3', '4', '_'],
                    ['5', '6', '7', '8', '9'],
                    ['_', 'A', 'B', 'C', '_'],
                    ['_', '_', 'D', '_', '_'],
                ][y][x]
            );
        }

        println!();
        panic!("See `stdout` for solution.")
    }
}
