use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use aoc::*;

#[derive(Clone, Debug)]
pub struct UnstableDiffusion(HashSet<Pos>);

impl Fro for UnstableDiffusion {
    fn fro(input: &str) -> Self {
        let mut grid = HashSet::new();

        for (y, row) in input.trim().split('\n').enumerate() {
            for (x, col) in row.trim().chars().enumerate() {
                let pos = Pos {
                    x: x as i64,
                    y: y as i64,
                };

                if col == '#' {
                    grid.insert(pos);
                }
            }
        }

        Self(grid)
    }
}

impl fmt::Display for UnstableDiffusion {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let ((min_x, max_x), (min_y, max_y)) = self.bounds();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.0.contains(&Pos { x, y }) {
                    write!(fmt, "#")?;
                } else {
                    write!(fmt, ".")?;
                }
            }
            writeln!(fmt)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
enum Proposal {
    Zero,
    One(Pos),
    Many,
}

impl Default for Proposal {
    fn default() -> Self {
        Proposal::Zero
    }
}

impl Proposal {
    fn propose(&mut self, elf: Pos) {
        match self {
            Proposal::Zero => *self = Proposal::One(elf),
            Proposal::One(_) => *self = Proposal::Many,
            Proposal::Many => (),
        }
    }
}

const DELTAS: [(i64, i64); 8] = [
    // NW
    (-1, -1),
    // N
    (0, -1),
    // NE
    (1, -1),
    // E
    (1, 0),
    // SE
    (1, 1),
    // S
    (0, 1),
    // SW
    (-1, 1),
    // W
    (-1, 0),
];

impl Solution for UnstableDiffusion {
    fn one(mut self) -> i64 {
        let mut directions = [[1, 2, 0], [5, 4, 6], [7, 0, 6], [3, 2, 4]];
        let mut proposals = HashMap::<Pos, Proposal>::new();
        let mut neighbors = [false; 8];

        for _ in 0..10 {
            for &Pos { x, y } in &self.0 {
                neighbors
                    .iter_mut()
                    .zip(DELTAS)
                    .for_each(|(neighbor, (dx, dy))| {
                        *neighbor = self.0.contains(&Pos {
                            x: x + dx,
                            y: y + dy,
                        });
                    });

                if neighbors.iter().all(|neighbor| !neighbor) {
                    continue;
                }

                for (index, deltas) in directions.into_iter().enumerate() {
                    if deltas.into_iter().all(|index| !neighbors[index]) {
                        let (dx, dy) = DELTAS[directions[index][0]];
                        proposals
                            .entry(Pos {
                                x: x + dx,
                                y: y + dy,
                            })
                            .or_default()
                            .propose(Pos { x, y });
                        break;
                    }
                }
            }

            for (new, proposal) in proposals.drain() {
                match proposal {
                    Proposal::Zero | Proposal::Many => (),
                    Proposal::One(old) => {
                        self.0.remove(&old);
                        self.0.insert(new);
                    }
                }
            }

            directions.rotate_left(1);
        }

        let ((min_x, max_x), (min_y, max_y)) = self.bounds();
        (max_y + 1 - min_y) * (max_x + 1 - min_x) - (self.0.len() as i64)
    }

    fn two(mut self) -> i64 {
        let mut directions = [[1, 2, 0], [5, 4, 6], [7, 0, 6], [3, 2, 4]];
        let mut proposals = HashMap::<Pos, Proposal>::new();
        let mut neighbors = [false; 8];

        for round in 1.. {
            for &Pos { x, y } in &self.0 {
                neighbors
                    .iter_mut()
                    .zip(DELTAS)
                    .for_each(|(neighbor, (dx, dy))| {
                        *neighbor = self.0.contains(&Pos {
                            x: x + dx,
                            y: y + dy,
                        });
                    });

                if neighbors.iter().all(|neighbor| !neighbor) {
                    continue;
                }

                for (index, deltas) in directions.into_iter().enumerate() {
                    if deltas.into_iter().all(|index| !neighbors[index]) {
                        let (dx, dy) = DELTAS[directions[index][0]];
                        proposals
                            .entry(Pos {
                                x: x + dx,
                                y: y + dy,
                            })
                            .or_default()
                            .propose(Pos { x, y });
                        break;
                    }
                }
            }

            let mut changed = false;

            for (new, proposal) in proposals.drain() {
                match proposal {
                    Proposal::Zero | Proposal::Many => (),
                    Proposal::One(old) => {
                        changed = true;
                        self.0.remove(&old);
                        self.0.insert(new);
                    }
                }
            }

            if !changed {
                return round as i64;
            }

            directions.rotate_left(1);
        }

        unreachable!()
    }
}

impl UnstableDiffusion {
    fn bounds(&self) -> ((i64, i64), (i64, i64)) {
        let mut min_x = i64::MAX;
        let mut min_y = i64::MAX;

        let mut max_x = i64::MIN;
        let mut max_y = i64::MIN;

        for &Pos { x, y } in &self.0 {
            min_x = cmp::min(min_x, x);
            min_y = cmp::min(min_y, y);

            max_x = cmp::max(max_x, x);
            max_y = cmp::max(max_y, y);
        }

        ((min_x, max_x), (min_y, max_y))
    }
}
