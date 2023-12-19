use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct ChronalCoordinates {
    min: Pos,
    max: Pos,
    grid: Vec<Pos>,
}

impl Fro for ChronalCoordinates {
    fn fro(input: &str) -> Self {
        let mut min = Pos::from_i_j(i64::MAX, i64::MAX);
        let mut max = Pos::from_i_j(i64::MIN, i64::MIN);
        let grid = input
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(", ").unwrap();
                let (x, y) = (x.to::<i64>(), y.to::<i64>());
                let pos = Pos { x, y };

                min = min.min(pos);
                max = max.max(pos);
                pos
            })
            .collect::<Vec<_>>();

        Self { min, max, grid }
    }
}

impl Solution for ChronalCoordinates {
    fn one(self) -> i64 {
        let rows = self.max.i() - self.min.i() + 1;
        let cols = self.max.j() - self.min.j() + 1;

        let mut owners = vec![(None, i64::MAX); (rows * cols) as usize];

        for (id, pos) in self.grid.iter().enumerate() {
            for p in self.min.to_inclusive(self.max) {
                let d = pos.distance_manhattan(p);
                let k = (p - self.min).to_index(cols);

                let (other, distance) = &mut owners[k];

                match (*distance).cmp(&d) {
                    Ordering::Less => (),
                    Ordering::Equal => *other = None,
                    Ordering::Greater => {
                        *other = Some(id);
                        *distance = d;
                    }
                }
            }
        }

        let cols = self.max.j() - self.min.j() + 1;

        let infinite = self
            .min
            .border_inclusive(self.max)
            .map(|pos| pos - self.min)
            .map(|pos| pos.to_index(cols))
            .map(|index| owners[index])
            .filter_map(fst)
            .collect::<HashSet<_>>();

        let mut counts = HashMap::new();

        owners
            .iter()
            .copied()
            .filter_map(fst)
            .filter(|owner| !infinite.contains(owner))
            .for_each(|owner| *counts.entry(owner).or_insert(0) += 1);

        counts.values().max().copied().unwrap() as i64
    }

    fn two(self) -> i64 {
        const EXPAND: i64 = 10_000;
        let expand = Pos::from_i_j(EXPAND, EXPAND);

        let min = self.min - expand;
        let max = self.max + expand;

        let rows = max.i() - min.i() + 1;
        let cols = max.j() - min.j() + 1;

        (0..(rows * cols) as usize)
            .map(|index| {
                let here = Pos::from_index(cols, index) + min;
                self.grid
                    .iter()
                    .map(|there| here.distance_manhattan(*there))
                    .sum::<i64>()
            })
            .filter(|distance| *distance < EXPAND)
            .count() as i64
    }
}
