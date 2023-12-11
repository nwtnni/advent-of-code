use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct CosmicExpansion {
    rows: i64,
    cols: i64,
    grid: HashSet<Pos>,
}

impl Fro for CosmicExpansion {
    fn fro(input: &str) -> Self {
        let mut grid = HashSet::new();
        let mut rows = 0;
        let mut cols = 0;
        for (y, row) in input.trim().split('\n').enumerate() {
            rows += 1;
            cols = 0;
            for (x, col) in row.trim().chars().enumerate() {
                cols += 1;
                let pos = Pos {
                    x: x as i64,
                    y: y as i64,
                };

                if col == '#' {
                    grid.insert(pos);
                }
            }
        }
        Self { rows, cols, grid }
    }
}

impl CosmicExpansion {
    fn expand(&self, by: i64) -> HashSet<Pos> {
        let is = (0..self.rows)
            .filter(|i| self.grid.iter().all(|pos| pos.y != *i))
            .collect::<Vec<_>>();
        let js = (0..self.cols)
            .filter(|j| self.grid.iter().all(|pos| pos.x != *j))
            .collect::<Vec<_>>();

        self.grid
            .iter()
            .map(|pos| Pos {
                x: pos.x + js.iter().filter(|j| **j < pos.x).count() as i64 * (by - 1),
                y: pos.y + is.iter().filter(|i| **i < pos.y).count() as i64 * (by - 1),
            })
            .collect::<HashSet<_>>()
    }
}

fn pairs(all: &HashSet<Pos>) -> i64 {
    let mut total = 0;
    for a in all.iter() {
        for b in all.iter() {
            if a == b {
                continue;
            }

            total += a.x.abs_diff(b.x);
            total += a.y.abs_diff(b.y);
        }
    }
    total as i64 / 2
}

impl Solution for CosmicExpansion {
    fn one(self) -> i64 {
        pairs(&self.expand(2))
    }

    fn two(self) -> i64 {
        pairs(&self.expand(1000000))
    }
}
