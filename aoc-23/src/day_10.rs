use std::collections::HashMap;
use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct PipeMaze {
    start: Pos,
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
        for (y, row) in input.trim().split('\n').enumerate() {
            for (x, col) in row.trim().chars().enumerate() {
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
        Self { start, grid }
    }
}

impl Solution for PipeMaze {
    fn one(mut self) -> i64 {
        let mut visited = HashSet::new();
        for pipe in PIPES {
            visited.clear();
            visited.insert(self.start);
            self.grid.insert(self.start, pipe);

            if let Some(distance) = self.search(&mut visited, self.start, self.start, 0) {
                return (distance + 1) / 2;
            }
        }

        unreachable!()
    }

    fn two(self) -> i64 {
        todo!()
    }
}

impl PipeMaze {
    fn search(
        &self,
        visited: &mut HashSet<Pos>,
        start: Pos,
        end: Pos,
        distance: i64,
    ) -> Option<i64> {
        let pipe = *self.grid.get(&start)?;

        for next in [
            if pipe & N > 0 {
                Some(start.shift(Dir::N))
            } else {
                None
            },
            if pipe & S > 0 {
                Some(start.shift(Dir::S))
            } else {
                None
            },
            if pipe & E > 0 {
                Some(start.shift(Dir::E))
            } else {
                None
            },
            if pipe & W > 0 {
                Some(start.shift(Dir::W))
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        {
            if next == end && distance > 1 {
                return Some(distance);
            }

            if !visited.insert(next) {
                continue;
            }

            if let Some(distance) = self.search(visited, next, end, distance + 1) {
                return Some(distance);
            }
        }

        None
    }
}
