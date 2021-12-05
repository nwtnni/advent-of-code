use std::cmp;
use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct HydrothermalVenture(Vec<[Pos; 2]>);

impl Fro for HydrothermalVenture {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (a, b) = line.split_once(" -> ").unwrap();
                let (x1, y1) = a.split_once(',').unwrap();
                let (x2, y2) = b.split_once(',').unwrap();
                [
                    Pos { x: i64::fro(x1), y: i64::fro(y1) },
                    Pos { x: i64::fro(x2), y: i64::fro(y2) },
                ]
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for HydrothermalVenture {
    fn one(self) -> i64 {
        let mut grid = HashMap::new();

        for [a, b] in self.0.into_iter().filter(|[a, b]| a.x == b.x || a.y == b.y) {
            for y in cmp::min(a.y, b.y)..=cmp::max(a.y, b.y) {
                for x in cmp::min(a.x, b.x)..=cmp::max(a.x, b.x) {
                    *grid.entry(Pos { x, y }).or_insert(0) += 1;
                }
            }
        }

        grid.values()
            .filter(|count| **count >= 2)
            .count()
            as i64
    }

    fn two(self) -> i64 {
        let mut grid = HashMap::new();

        for [mut a, b] in self.0 {
            let dx = match a.x.cmp(&b.x) {
                cmp::Ordering::Less => 1,
                cmp::Ordering::Equal => 0,
                cmp::Ordering::Greater => -1,
            };

            let dy = match a.y.cmp(&b.y) {
                cmp::Ordering::Less => 1,
                cmp::Ordering::Equal => 0,
                cmp::Ordering::Greater => -1,
            };

            loop {
                *grid.entry(a).or_insert(0) += 1;

                if a == b {
                    break;
                }

                a.x += dx;
                a.y += dy;
            }
        }

        grid.values()
            .filter(|count| **count >= 2)
            .count()
            as i64
    }
}
