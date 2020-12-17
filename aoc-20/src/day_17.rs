use std::collections::HashSet;
use std::cmp;
use std::mem;

use aoc::*;

#[derive(Clone, Debug)]
pub struct ConwayCubes {
    cols: usize,
    rows: usize,
    grid: HashSet<(i64, i64, i64, i64)>,
}

impl Fro for ConwayCubes {
    fn fro(input: &str) -> Self {
        let mut grid = HashSet::new();
        let mut rows = 0;
        let mut cols = 0;
        for (y, row) in input.trim().split('\n').enumerate() {
            rows += 1;
            cols = 0;
            for (x, col) in row.trim().chars().enumerate() {
                cols += 1;

                if col == '#' {
                    grid.insert((x as i64, y as i64, 0, 0));
                }

            }
        }
        Self {
            cols,
            rows,
            grid,
        }
    }
}

impl Solution for ConwayCubes {
    fn one(mut self) -> i64 {
        let mut grid = HashSet::new();
        for _ in 0..6 {
            grid.clear();

            let mut min_x = i64::MAX;
            let mut max_x = i64::MIN;

            let mut min_y = i64::MAX;
            let mut max_y = i64::MIN;

            let mut min_z = i64::MAX;
            let mut max_z = i64::MIN;

            for (x, y, z, _) in &self.grid {
                min_x = cmp::min(min_x, *x);
                max_x = cmp::max(max_x, *x);

                min_y = cmp::min(min_y, *y);
                max_y = cmp::max(max_y, *y);

                min_z = cmp::min(min_z, *z);
                max_z = cmp::max(max_z, *z);
            }

            for x in min_x - 1..=max_x + 1 {
                for y in min_y - 1..=max_y + 1 {
                    for z in min_z - 1..=max_z + 1 {

                        let mut around = 0;

                        for dx in -1i64..=1 {
                            for dy in -1i64..=1 {
                                for dz in -1i64..=1 {
                                    if dx == 0 && dy == 0 && dz == 0 {
                                        continue;
                                    }

                                    if self.grid.contains(&(x + dx, y + dy, z + dz, 0)) {
                                        around += 1;
                                    }
                                }
                            }
                        }

                        match (self.grid.contains(&(x, y, z, 0)), around) {
                        | (true, 2..=3) => { grid.insert((x, y, z, 0)); },
                        | (true, _) => (),
                        | (false, 3) => { grid.insert((x, y, z, 0)); },
                        | (false, _) => (),
                        }
                    }
                }
            }

            mem::swap(&mut grid, &mut self.grid);
        }

        self.grid.len() as i64
    }

    fn two(mut self) -> i64 {
        let mut grid = HashSet::new();
        for _ in 0..6 {
            grid.clear();

            let mut min_x = i64::MAX;
            let mut max_x = i64::MIN;

            let mut min_y = i64::MAX;
            let mut max_y = i64::MIN;

            let mut min_z = i64::MAX;
            let mut max_z = i64::MIN;

            let mut min_w = i64::MAX;
            let mut max_w = i64::MIN;

            for (x, y, z, w) in &self.grid {
                min_x = cmp::min(min_x, *x);
                max_x = cmp::max(max_x, *x);

                min_y = cmp::min(min_y, *y);
                max_y = cmp::max(max_y, *y);

                min_z = cmp::min(min_z, *z);
                max_z = cmp::max(max_z, *z);

                min_w = cmp::min(min_w, *w);
                max_w = cmp::max(max_w, *w);
            }

            for x in min_x - 1..=max_x + 1 {
                for y in min_y - 1..=max_y + 1 {
                    for z in min_z - 1..=max_z + 1 {
                        for w in min_w - 1..=max_w + 1 {

                            let mut around = 0;

                            for dx in -1i64..=1 {
                                for dy in -1i64..=1 {
                                    for dz in -1i64..=1 {
                                        for dw in -1i64..=1 {
                                            if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                                                continue;
                                            }

                                            if self.grid.contains(&(x + dx, y + dy, z + dz, w + dw)) {
                                                around += 1;
                                            }
                                        }
                                    }
                                }
                            }

                            match (self.grid.contains(&(x, y, z, w)), around) {
                            | (true, 2..=3) => { grid.insert((x, y, z, w)); },
                            | (true, _) => (),
                            | (false, 3) => { grid.insert((x, y, z, w)); },
                            | (false, _) => (),
                            }
                        }
                    }
                }
            }

            mem::swap(&mut grid, &mut self.grid);
        }

        self.grid.len() as i64
    }
}
