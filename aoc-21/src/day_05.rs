use std::cmp;
use std::collections::HashMap;
use std::iter;

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

        self
            .0
            .iter()
            .flat_map(|[a, b]| {
                match (a.x.cmp(&b.x), a.y.cmp(&b.y)) {
                    (cmp::Ordering::Equal, _) => (cmp::min(a.y, b.y)..=cmp::max(a.y, b.y))
                        .map(move |y| Pos { x: a.x, y })
                        .tap(Box::new)
                        as Box<dyn Iterator<Item = _>>,
                    (_, cmp::Ordering::Equal) => (cmp::min(a.x, b.x)..=cmp::max(a.x, b.x))
                        .map(move |x| Pos { x, y: a.y })
                        .tap(Box::new),
                    (_, _) => iter::empty()
                        .tap(Box::new),
                }
            })
            .for_each(|pos| *grid.entry(pos).or_insert(0) += 1);

        grid.values()
            .filter(|count| **count >= 2)
            .count()
            as i64
    }

    fn two(self) -> i64 {
        let mut grid = HashMap::new();

        self
            .0
            .iter()
            .flat_map(|[a, b]| {
                match (a.x.cmp(&b.x), a.y.cmp(&b.y)) {
                    (cmp::Ordering::Equal, _) => (cmp::min(a.y, b.y)..=cmp::max(a.y, b.y))
                        .map(move |y| Pos { x: a.x, y })
                        .tap(Box::new)
                        as Box<dyn Iterator<Item = _>>,
                    (_, cmp::Ordering::Equal) => (cmp::min(a.x, b.x)..=cmp::max(a.x, b.x))
                        .map(move |x| Pos { x, y: a.y })
                        .tap(Box::new),
                    (cmp::Ordering::Less, cmp::Ordering::Less) => (a.x..=b.x)
                        .zip(a.y..=b.y)
                        .map(|(x, y)| Pos { x, y })
                        .tap(Box::new),
                    (cmp::Ordering::Less, cmp::Ordering::Greater) => (a.x..=b.x)
                        .zip((b.y..=a.y).rev())
                        .map(|(x, y)| Pos { x, y })
                        .tap(Box::new),
                    (cmp::Ordering::Greater, cmp::Ordering::Less) => ((b.x..=a.x).rev())
                        .zip(a.y..=b.y)
                        .map(|(x, y)| Pos { x, y })
                        .tap(Box::new),
                    (cmp::Ordering::Greater, cmp::Ordering::Greater) => ((b.x..=a.x).rev())
                        .zip((b.y..=a.y).rev())
                        .map(|(x, y)| Pos { x, y })
                        .tap(Box::new),
                }
            })
            .for_each(|pos| *grid.entry(pos).or_insert(0) += 1);

        grid.values()
            .filter(|count| **count >= 2)
            .count()
            as i64
    }
}
