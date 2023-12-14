use std::collections::BTreeMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct ParabolicReflectorDish {
    cols: usize,
    rows: usize,
    grid: BTreeMap<Pos, bool>,
}

impl Fro for ParabolicReflectorDish {
    fn fro(input: &str) -> Self {
        let mut grid = BTreeMap::new();
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
        self.score()
    }

    fn two(mut self) -> i64 {
        let mut seen = BTreeMap::new();
        seen.insert(self.grid.clone(), 0);

        let (start, end) = loop {
            for dir in [Dir::N, Dir::W, Dir::S, Dir::E] {
                self.tilt(dir);
            }
            let end = seen.len();
            if let Some(start) = seen.insert(self.grid.clone(), end) {
                break (start, end);
            }
        };

        seen.iter()
            .find(|(_, index)| **index == ((1000000000 - start) % (end - start)) + start)
            .map(|(map, _)| {
                map.iter()
                    .map(|(pos, rock)| if *rock { self.rows as i64 - pos.y } else { 0 })
                    .sum::<i64>()
            })
            .unwrap()
    }
}

impl ParabolicReflectorDish {
    fn tilt(&mut self, dir: Dir) {
        loop {
            let mut changed = false;
            for i in 0..self.rows as i64 {
                for j in 0..self.cols as i64 {
                    let a = Pos { x: j, y: i };
                    let b = a.shift(dir);

                    if b.x < 0 || b.x >= self.cols as i64 || b.y < 0 || b.y >= self.rows as i64 {
                        continue;
                    }

                    if let (Some(true), None) =
                        (self.grid.get(&a).copied(), self.grid.get(&b).copied())
                    {
                        changed = true;
                        self.grid.remove(&a);
                        self.grid.insert(b, true);
                    }
                }
            }

            if !changed {
                break;
            }
        }
    }

    fn score(&self) -> i64 {
        self.grid
            .iter()
            .map(|(pos, rock)| if *rock { self.rows as i64 - pos.y } else { 0 })
            .sum::<i64>()
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
