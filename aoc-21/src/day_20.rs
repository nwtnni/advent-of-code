use std::collections::HashMap;
use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct TrenchMap {
    cols: usize,
    rows: usize,
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

        assert_eq!(enhance.len(), 512);

        let mut lines = iter.give();

        let mut grid = HashMap::new();
        let mut rows = 0;
        let mut cols = 0;
        for (y, row) in lines.trim().split('\n').enumerate() {
            rows += 1;
            cols = 0;
            for (x, col) in row.trim().chars().enumerate() {
                cols += 1;
                let pos = Pos {
                    x: x as i64,
                    y: y as i64,
                };

                let cell = match col {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!(),
                };

                grid.insert(pos, cell);
            }
        }
        Self {
            cols,
            rows,
            grid,
            enhance,
        }
    }
}

impl Solution for TrenchMap {
    fn one(mut self) -> i64 {
        let mut next = HashMap::new();

        for round in 0..2 {
            let min_x = self.grid.keys().map(|pos| pos.x).min().unwrap();
            let min_y = self.grid.keys().map(|pos| pos.y).min().unwrap();

            let max_x = self.grid.keys().map(|pos| pos.x).max().unwrap();
            let max_y = self.grid.keys().map(|pos| pos.y).max().unwrap();

            next.clear();

            for i in min_y - 3..=max_y + 3 {
                for j in min_x - 3..=max_x + 3 {
                    let mut index = 0;
                    let mut bit = 8;

                    for di in -1..=1 {
                        for dj in -1..=1 {
                            index |= (*self
                                .grid
                                .get(&Pos {
                                    x: j + dj,
                                    y: i + di,
                                })
                                .unwrap_or(&if round % 2 == 0 { false } else { true })
                                as usize)
                                << bit;
                            bit -= 1;
                        }
                    }

                    next.insert(Pos { x: j, y: i }, self.enhance[index]);
                }
            }

            std::mem::swap(&mut self.grid, &mut next);
        }

        self.grid.values().filter(|x| **x).count() as i64
    }

    fn two(mut self) -> i64 {
        let mut next = HashMap::new();

        for round in 0..50 {
            let min_x = self.grid.keys().map(|pos| pos.x).min().unwrap();
            let min_y = self.grid.keys().map(|pos| pos.y).min().unwrap();

            let max_x = self.grid.keys().map(|pos| pos.x).max().unwrap();
            let max_y = self.grid.keys().map(|pos| pos.y).max().unwrap();

            next.clear();

            for i in min_y - 3..=max_y + 3 {
                for j in min_x - 3..=max_x + 3 {
                    let mut index = 0;
                    let mut bit = 8;

                    for di in -1..=1 {
                        for dj in -1..=1 {
                            index |= (*self
                                .grid
                                .get(&Pos {
                                    x: j + dj,
                                    y: i + di,
                                })
                                .unwrap_or(&if round % 2 == 0 { false } else { true })
                                as usize)
                                << bit;
                            bit -= 1;
                        }
                    }

                    next.insert(Pos { x: j, y: i }, self.enhance[index]);
                }
            }

            std::mem::swap(&mut self.grid, &mut next);
        }

        self.grid.values().filter(|x| **x).count() as i64
    }
}
