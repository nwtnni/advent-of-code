use std::collections::HashMap;
use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
#[allow(unused)]
pub struct HoofIt {
    cols: usize,
    rows: usize,
    grid: HashMap<Pos, u8>,
}

impl Fro for HoofIt {
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

                grid.insert(pos, col as u8 - b'0');
            }
        }
        Self { cols, rows, grid }
    }
}

impl Solution for HoofIt {
    fn one(self) -> i64 {
        let mut out = HashSet::new();
        self.grid
            .iter()
            .filter(|(_, h)| **h == 0)
            .map(|(p, _)| {
                out.clear();
                self.reachable(*p, &mut out);
                out.len() as i64
            })
            .sum()
    }

    fn two(self) -> i64 {
        self.grid
            .iter()
            .filter(|(_, h)| **h == 0)
            .map(|(p, _)| self.paths(*p))
            .sum()
    }
}

impl HoofIt {
    fn reachable(&self, p: Pos, out: &mut HashSet<Pos>) {
        match self.grid.get(&p) {
            None => (),
            Some(9) => {
                out.insert(p);
            }
            Some(l) => p
                .around_manhattan_exclusive(1)
                .filter(|q| matches!(self.grid.get(q), Some(h) if *h == l + 1))
                .for_each(|q| self.reachable(q, out)),
        }
    }

    fn paths(&self, p: Pos) -> i64 {
        match self.grid.get(&p) {
            None => 0,
            Some(9) => 1,
            Some(l) => p
                .around_manhattan_exclusive(1)
                .filter(|q| matches!(self.grid.get(q), Some(h) if *h == l + 1))
                .map(|q| self.paths(q))
                .sum(),
        }
    }
}
