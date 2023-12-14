use std::collections::BTreeMap;
use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct ParabolicReflectorDish {
    cols: usize,
    rows: usize,
    grid: HashMap<Pos, bool>,
}

impl Fro for ParabolicReflectorDish {
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

                let cell = match col {
                    'O' => true,
                    '#' => false,
                    _ => continue,
                };

                grid.insert(pos, cell);
            }
        }
        Self { cols, rows, grid }
    }
}

impl Solution for ParabolicReflectorDish {
    fn one(mut self) -> i64 {
        self.tilt(Dir::N);

        self.grid
            .iter()
            .map(|(pos, rock)| if *rock { self.rows as i64 - pos.y } else { 0 })
            .sum::<i64>()
    }

    fn two(mut self) -> i64 {
        let mut seen = BTreeMap::new();
        seen.insert(
            (self
                .grid
                .iter()
                .map(|(p, b)| (*p, *b))
                .collect::<BTreeMap<_, _>>(),),
            0,
        );

        let (start, end) = loop {
            for dir in [Dir::N, Dir::W, Dir::S, Dir::E] {
                self.tilt(dir);
            }
            let end = seen.len();
            let start = seen.insert(
                (self
                    .grid
                    .iter()
                    .map(|(p, b)| (*p, *b))
                    .collect::<BTreeMap<_, _>>(),),
                end,
            );

            if seen.len() == end {
                break (start, end);
            }
        };

        let index = ((1000000000 - start.unwrap()) % (end - start.unwrap())) + start.unwrap();
        seen.iter()
            .find(|(_, key)| **key == index)
            .map(|(map, _)| {
                map.0
                    .iter()
                    .map(|(pos, rock)| if *rock { self.rows as i64 - pos.y } else { 0 })
                    .sum::<i64>()
            })
            .unwrap()
    }
}

impl ParabolicReflectorDish {
    fn tilt(&mut self, dir: Dir) {
        loop {
            let old = self.grid.clone();
            for i in 0..self.rows as i64 {
                for j in 0..self.cols as i64 {
                    let p = Pos { x: j, y: i };
                    match (
                        self.grid.get(&p).copied(),
                        self.grid.get(&p.shift(dir)).copied(),
                    ) {
                        (None, None) => (),
                        (None, Some(_)) => (),
                        (Some(true), None) => {
                            let p = p.shift(dir);
                            if p.x >= 0
                                && p.x < self.cols as i64
                                && p.y >= 0
                                && p.y < self.rows as i64
                            {
                                self.grid.remove(&Pos { x: j, y: i });
                                self.grid.insert(p, true);
                            }
                        }
                        (Some(false), None) => (),
                        (Some(_), Some(_)) => (),
                    }
                }
            }
            if self.grid == old {
                break;
            }
        }
    }

    #[allow(dead_code)]
    fn debug(&self) {
        for i in 0..self.rows as i64 {
            for j in 0..self.cols as i64 {
                match self.grid.get(&Pos { x: j, y: i }) {
                    None => print!("."),
                    Some(true) => print!("O"),
                    Some(false) => print!("#"),
                }
            }
            println!();
        }
        println!();
    }
}
