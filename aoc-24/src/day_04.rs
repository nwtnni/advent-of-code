use std::collections::HashMap;
use std::iter;

use aoc::*;

#[derive(Clone, Debug)]
pub struct CeresSearch {
    cols: usize,
    rows: usize,
    grid: HashMap<Pos, char>,
}

impl Fro for CeresSearch {
    fn fro(input: &str) -> Self {
        let mut grid = HashMap::new();
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

                grid.insert(pos, col);
            }
        }
        Self { cols, rows, grid }
    }
}

impl Solution for CeresSearch {
    fn one(self) -> i64 {
        let mut count = 0;
        for y in 0..self.rows as i64 {
            for x in 0..self.cols as i64 {
                // Cardinal
                for dir in Dir::all() {
                    count += self.line(Pos { x, y }, |pos| pos.shift(dir)) as i64;
                }

                // Diagonal
                for v in [Dir::N, Dir::S] {
                    for h in [Dir::E, Dir::W] {
                        count += self.line(Pos { x, y }, |pos| pos.shift(v).shift(h)) as i64;
                    }
                }
            }
        }
        count
    }

    fn two(self) -> i64 {
        let mut count = 0;
        for y in 0..self.rows as i64 {
            for x in 0..self.cols as i64 {
                count += self.cross(Pos { x, y }) as i64;
            }
        }
        count
    }
}

impl CeresSearch {
    fn line<F: FnMut(Pos) -> Pos>(&self, mut pos: Pos, mut shift: F) -> bool {
        iter::from_fn(|| {
            let next = self.grid.get(&pos);
            pos = shift(pos);
            next
        })
        .take(4)
        .copied()
        .eq(['X', 'M', 'A', 'S'])
    }

    fn cross(&self, pos: Pos) -> bool {
        if !matches!(self.grid.get(&pos), Some('A')) {
            return false;
        }

        [
            (
                pos.shift(Dir::N).shift(Dir::W),
                pos.shift(Dir::S).shift(Dir::E),
            ),
            (
                pos.shift(Dir::N).shift(Dir::E),
                pos.shift(Dir::S).shift(Dir::W),
            ),
        ]
        .into_iter()
        .map(|(p, q)| self.grid.get(&p).zip(self.grid.get(&q)))
        .map(|pair| pair.map(|(&p, &q)| p == 'M' && q == 'S' || p == 'S' && q == 'M'))
        .try_fold(true, |all, next| Some(all && next?))
        .unwrap_or(false)
    }
}
