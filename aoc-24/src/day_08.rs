use std::collections::HashMap;
use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct ResonantCollinearity {
    cols: usize,
    rows: usize,
    grid: HashMap<Pos, char>,
}

impl Fro for ResonantCollinearity {
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

                if col != '.' {
                    grid.insert(pos, col);
                }
            }
        }
        Self { cols, rows, grid }
    }
}

impl Solution for ResonantCollinearity {
    fn one(self) -> i64 {
        let mut fs = HashMap::new();
        self.grid
            .iter()
            .for_each(|(pos, f)| fs.entry(f).or_insert_with(Vec::new).push(pos));

        let mut nodes = HashSet::new();
        for ps in fs.values() {
            for i in 0..ps.len() {
                for j in i + 1..ps.len() {
                    let pi = ps[i];
                    let pj = ps[j];

                    // Segment from ps[i] to ps[j]
                    let dx = pj.x - pi.x;
                    let dy = pj.y - pi.y;

                    nodes.insert(Pos {
                        x: pj.x + dx,
                        y: pj.y + dy,
                    });

                    nodes.insert(Pos {
                        x: pi.x - dx,
                        y: pi.y - dy,
                    });
                }
            }
        }

        nodes
            .iter()
            .filter(|p| {
                (0..self.cols as i64).contains(&p.x) && (0..self.rows as i64).contains(&p.y)
            })
            .count() as i64
    }

    fn two(self) -> i64 {
        let mut fs = HashMap::new();
        self.grid
            .iter()
            .for_each(|(pos, f)| fs.entry(f).or_insert_with(Vec::new).push(pos));

        let mut nodes = HashSet::new();
        for ps in fs.values() {
            for i in 0..ps.len() {
                for j in i + 1..ps.len() {
                    let pi = ps[i];
                    let pj = ps[j];

                    // Segment from ps[i] to ps[j]
                    let dx = pj.x - pi.x;
                    let dy = pj.y - pi.y;

                    nodes.insert(*pi);
                    nodes.insert(*pj);

                    // i to j
                    nodes.extend(
                        (1..)
                            .map(|d| Pos {
                                x: pj.x + d * dx,
                                y: pj.y + d * dy,
                            })
                            .take_while(|p| self.contains(*p)),
                    );

                    // j to i
                    nodes.extend(
                        (1..)
                            .map(|d| Pos {
                                x: pi.x - d * dx,
                                y: pi.y - d * dy,
                            })
                            .take_while(|p| self.contains(*p)),
                    );
                }
            }
        }

        nodes.len() as i64
    }
}

impl ResonantCollinearity {
    fn contains(&self, p: Pos) -> bool {
        (0..self.cols as i64).contains(&p.x) && (0..self.rows as i64).contains(&p.y)
    }
}
