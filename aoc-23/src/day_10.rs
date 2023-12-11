use std::collections::HashMap;
use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct PipeMaze {
    start: Pos,
    rows: i64,
    cols: i64,
    grid: HashMap<Pos, Pipe>,
}

type Pipe = u8;

const N: u8 = 0b0001;
const S: u8 = 0b0010;
const E: u8 = 0b0100;
const W: u8 = 0b1000;

const PIPES: [Pipe; 6] = [N | S, E | W, N | E, N | W, S | E, S | W];

impl Fro for PipeMaze {
    fn fro(input: &str) -> Self {
        let mut grid = HashMap::new();
        let mut start = Pos::default();
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

                let cell = match col {
                    '|' => N | S,
                    '-' => E | W,
                    'L' => N | E,
                    'J' => N | W,
                    '7' => S | W,
                    'F' => S | E,
                    '.' => continue,
                    'S' => {
                        start = pos;
                        continue;
                    }
                    _ => unreachable!(),
                };

                grid.insert(pos, cell);
            }
        }
        Self {
            start,
            rows,
            cols,
            grid,
        }
    }
}

impl Solution for PipeMaze {
    fn one(mut self) -> i64 {
        for pipe in PIPES {
            self.grid.insert(self.start, pipe);

            if let Some(pipes) = self.trace(self.start) {
                return (pipes.len() as i64 + 1) / 2;
            }
        }

        unreachable!()
    }

    fn two(mut self) -> i64 {
        for pipe in PIPES {
            self.grid.insert(self.start, pipe);

            if let Some(pipes) = self.trace(self.start) {
                let mut total = 0;
                for i in 0..self.rows {
                    let mut inside = false;

                    for j in 0..self.cols {
                        match pipes.get(&Pos { x: j, y: i }) {
                            None => total += inside as i64,
                            Some(pos) => {
                                inside ^= self.grid[pos] & N > 0;
                            }
                        }
                    }
                }

                return total;
            }
        }

        unreachable!()
    }
}

impl PipeMaze {
    fn trace(&self, start: Pos) -> Option<HashSet<Pos>> {
        let mut visited = HashSet::new();
        visited.insert(start);

        let mut here = start;

        'outer: loop {
            let pipe = *self.grid.get(&here)?;

            'inner: for (next, _) in [
                if pipe & N > 0 {
                    Some((here.shift(Dir::N), S))
                } else {
                    None
                },
                if pipe & S > 0 {
                    Some((here.shift(Dir::S), N))
                } else {
                    None
                },
                if pipe & E > 0 {
                    Some((here.shift(Dir::E), W))
                } else {
                    None
                },
                if pipe & W > 0 {
                    Some((here.shift(Dir::W), E))
                } else {
                    None
                },
            ]
            .into_iter()
            .flatten()
            .filter(|(next, _)| self.grid.contains_key(next))
            .filter(|(next, opposite)| self.grid[next] & opposite > 0)
            {
                if visited.len() > 2 && next == start {
                    return Some(visited);
                }

                if visited.insert(next) {
                    here = next;
                    continue 'outer;
                } else {
                    continue 'inner;
                }
            }

            return None;
        }
    }
}
