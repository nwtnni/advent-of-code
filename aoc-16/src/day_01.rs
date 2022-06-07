use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct NoTimeForATaxicab(Vec<(Turn, i64)>);

#[derive(Copy, Clone, Debug)]
enum Turn {
    L,
    R,
}

impl Fro for NoTimeForATaxicab {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split(", ")
            .map(|instruction| {
                let turn = match instruction.chars().give() {
                    'L' => Turn::L,
                    'R' => Turn::R,
                    _ => unreachable!(),
                };

                let blocks = i64::fro(&instruction[1..]);

                (turn, blocks)
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for NoTimeForATaxicab {
    fn one(self) -> i64 {
        let mut pos = Pos { x: 0, y: 0 };
        let mut dir = Dir::N;

        for (turn, blocks) in &self.0 {
            match turn {
                Turn::L => dir.rotate_counterclockwise_mut(),
                Turn::R => dir.rotate_clockwise_mut(),
            }

            for _ in 0..*blocks {
                pos.shift_mut(dir);
            }
        }

        pos.x.abs() + pos.y.abs()
    }

    fn two(self) -> i64 {
        let mut pos = Pos { x: 0, y: 0 };
        let mut dir = Dir::N;
        let mut set = HashSet::new();

        set.insert(pos);

        for (turn, blocks) in &self.0 {
            match turn {
                Turn::L => dir.rotate_counterclockwise_mut(),
                Turn::R => dir.rotate_clockwise_mut(),
            }

            for _ in 0..*blocks {
                pos.shift_mut(dir);

                if !set.insert(pos) {
                    return pos.x.abs() + pos.y.abs();
                }
            }
        }

        unreachable!()
    }
}
