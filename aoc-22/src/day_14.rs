use std::cmp;
use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct RegolithReservoir(HashSet<(i64, i64)>);

impl Fro for RegolithReservoir {
    fn fro(input: &str) -> Self {
        let mut grid = HashSet::new();

        for line in input.trim().split('\n') {
            let points = line
                .split(" -> ")
                .map(|xy| {
                    let (x, y) = xy.split_once(',').unwrap();
                    (i64::fro(x), i64::fro(y))
                })
                .collect::<Vec<_>>();

            for window in points.windows(2) {
                let (x1, y1) = window[0];
                let (x2, y2) = window[1];
                let points = if x1 == x2 {
                    (cmp::min(y1, y2)..=cmp::max(y1, y2))
                        .map(move |y| (x1, y))
                        .tap(Or::L)
                } else {
                    (cmp::min(x1, x2)..=cmp::max(x1, x2))
                        .map(move |x| (x, y1))
                        .tap(Or::R)
                };

                grid.extend(points);
            }
        }

        Self(grid)
    }
}

impl Solution for RegolithReservoir {
    fn one(mut self) -> i64 {
        let void = self.0.iter().map(|(_, y)| *y).max().unwrap();
        let mut total = 0;

        loop {
            let mut x = 500;
            let mut y = 0;

            loop {
                if y > void {
                    return total;
                }

                if self.0.get(&(x, y + 1)).is_none() {
                    y += 1;
                } else if self.0.get(&(x - 1, y + 1)).is_none() {
                    x -= 1;
                    y += 1;
                } else if self.0.get(&(x + 1, y + 1)).is_none() {
                    x += 1;
                    y += 1;
                } else {
                    total += 1;
                    self.0.insert((x, y));
                    break;
                }
            }
        }
    }

    fn two(mut self) -> i64 {
        let floor = self.0.iter().map(|(_, y)| *y).max().unwrap();
        let mut total = 0;

        loop {
            let mut x = 500;
            let mut y = 0;

            loop {
                let above = y + 1 < floor + 2;

                if self.0.get(&(x, y + 1)).is_none() && above {
                    y += 1;
                } else if self.0.get(&(x - 1, y + 1)).is_none() && above {
                    x -= 1;
                    y += 1;
                } else if self.0.get(&(x + 1, y + 1)).is_none() && above {
                    x += 1;
                    y += 1;
                } else {
                    total += 1;
                    self.0.insert((x, y));

                    if x == 500 && y == 0 {
                        return total;
                    }

                    break;
                }
            }
        }
    }
}
