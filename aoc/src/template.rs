use std::collections::HashMap;
use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct $TITLE(Vec<...>);
pub struct $TITLE(HashSet<...>);
pub struct $TITLE {
    cols: usize,
    rows: usize,
    grid: HashMap<Pos, ...>,
}

impl Fro for $TITLE {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {

            })
            .collect::<Vec<_>>()
            // .collect::<HashMap<_, _>>()
            // .collect::<HashSet<_>>()
            .tap(Self)
    }
}

impl Fro for $TITLE {
    fn fro(input: &str) -> Self {
        let mut grid = HashMap::new();
        let mut rows = 0;
        let mut cols = 0;
        for (y, row) in input.trim().split('\n').enumerate() {
            rows += 1;
            cols = 0;
            for (x, col) in row.trim().chars() {
                cols += 1;
                let pos = Pos {
                    x: x as i64,
                    y: y as i64,
                };

                let cell = match col {

                };

                grid.insert(pos, cell);
            }
        }
        Self(grid)
    }
}

impl Solution for $TITLE {
    fn one(self) -> i64 {

    }

    fn two(self) -> i64 {
        todo!()
    }
}
