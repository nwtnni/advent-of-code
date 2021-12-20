use std::cmp;
use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct TrenchMap {
    grid: HashMap<Pos, bool>,
    enhance: Vec<bool>,
}

impl Fro for TrenchMap {
    fn fro(input: &str) -> Self {
        let mut iter = input.trim().split("\n\n");

        let enhance = iter
            .give()
            .trim()
            .chars()
            .map(|char| char == '#')
            .collect::<Vec<_>>();

        let lines = iter.give();
        let mut grid = HashMap::new();

        for (y, row) in lines.trim().split('\n').enumerate() {
            for (x, col) in row.trim().chars().enumerate() {
                let pos = Pos {
                    x: x as i64,
                    y: y as i64,
                };

                grid.insert(pos, col == '#');
            }
        }

        Self { grid, enhance }
    }
}

impl Solution for TrenchMap {
    fn one(mut self) -> i64 {
        self.enhance(2)
    }

    fn two(mut self) -> i64 {
        self.enhance(50)
    }
}

impl TrenchMap {
    fn enhance(&mut self, rounds: usize) -> i64 {
        let mut buffer = HashMap::new();

        for round in 0..rounds {
            buffer.clear();

            let (min_i, max_i, min_j, max_j) = self.grid.keys().fold(
                (i64::MAX, i64::MIN, i64::MAX, i64::MIN),
                |(min_i, max_i, min_j, max_j), Pos { x, y }| {
                    (
                        cmp::min(min_i, *y),
                        cmp::max(max_i, *y),
                        cmp::min(min_j, *x),
                        cmp::max(max_j, *x),
                    )
                },
            );

            for i in min_i - 2..=max_i + 2 {
                for j in min_j - 2..=max_j + 2 {
                    let mut index = 0;
                    let mut bit = 8;

                    for di in -1..=1 {
                        for dj in -1..=1 {
                            let on = self
                                .grid
                                .get(&Pos {
                                    x: j + dj,
                                    y: i + di,
                                })
                                .copied()
                                .unwrap_or(round % 2 == 1);
                            index |= (on as usize) << bit;
                            bit -= 1;
                        }
                    }

                    buffer.insert(Pos { x: j, y: i }, self.enhance[index]);
                }
            }

            std::mem::swap(&mut self.grid, &mut buffer);
        }

        self.grid.values().filter(|on| **on).count() as i64
    }
}
