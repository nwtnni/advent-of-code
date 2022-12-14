use std::cmp;
use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct RegolithReservoir(Vec<Vec<(i64, i64)>>);

impl Fro for RegolithReservoir {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                line.split(" -> ")
                    .map(|xy| {
                        let (x, y) = xy.split_once(',').unwrap();
                        (i64::fro(x), i64::fro(y))
                    })
                    .collect()
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

#[derive(Debug)]
enum Tile {
    Wall,
    Sand,
}

impl Solution for RegolithReservoir {
    fn one(self) -> i64 {
        let mut grid = HashMap::new();
        for line in self.0 {
            for points in line.windows(2) {
                // x same, y different
                if points[0].0 == points[1].0 {
                    for y in cmp::min(points[0].1, points[1].1)..=cmp::max(points[0].1, points[1].1)
                    {
                        grid.insert((points[0].0, y), Tile::Wall);
                    }
                } else {
                    for x in cmp::min(points[0].0, points[1].0)..=cmp::max(points[0].0, points[1].0)
                    {
                        grid.insert((x, points[0].1), Tile::Wall);
                    }
                }
            }
        }

        let low = grid.keys().map(|(_, y)| *y).max().unwrap();
        let mut total = 0;

        loop {
            let mut x = 500;
            let mut y = 0;

            loop {
                if y > low {
                    return total;
                }

                if grid.get(&(x, y + 1)).is_none() {
                    y += 1;
                } else if grid.get(&(x - 1, y + 1)).is_none() {
                    x -= 1;
                    y += 1;
                } else if grid.get(&(x + 1, y + 1)).is_none() {
                    x += 1;
                    y += 1;
                } else {
                    total += 1;
                    grid.insert((x, y), Tile::Sand);
                    break;
                }
            }
        }
    }

    fn two(self) -> i64 {
        let mut grid = HashMap::new();
        for line in self.0 {
            for points in line.windows(2) {
                // x same, y different
                if points[0].0 == points[1].0 {
                    for y in cmp::min(points[0].1, points[1].1)..=cmp::max(points[0].1, points[1].1)
                    {
                        grid.insert((points[0].0, y), Tile::Wall);
                    }
                } else {
                    for x in cmp::min(points[0].0, points[1].0)..=cmp::max(points[0].0, points[1].0)
                    {
                        grid.insert((x, points[0].1), Tile::Wall);
                    }
                }
            }
        }

        let low = grid.keys().map(|(_, y)| *y).max().unwrap();
        let mut total = 0;

        loop {
            let mut x = 500;
            let mut y = 0;

            loop {
                if grid.get(&(x, y + 1)).is_none() && y + 1 < low + 2 {
                    y += 1;
                } else if grid.get(&(x - 1, y + 1)).is_none() && y + 1 < low + 2 {
                    x -= 1;
                    y += 1;
                } else if grid.get(&(x + 1, y + 1)).is_none() && y + 1 < low + 2 {
                    x += 1;
                    y += 1;
                } else {
                    total += 1;
                    grid.insert((x, y), Tile::Sand);

                    if x == 500 && y == 0 {
                        return total;
                    }

                    break;
                }
            }
        }
    }
}
