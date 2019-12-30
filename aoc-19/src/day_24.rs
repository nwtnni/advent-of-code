use std::collections::BTreeSet;
use std::collections::HashSet;
use std::mem;

use aoc::*;

pub struct PlanetOfDiscord {
    prev: BTreeSet<Pos>,
    next: BTreeSet<Pos>,
}

impl Fro for PlanetOfDiscord {
    fn fro(input: &str) -> Self {
        let mut prev = BTreeSet::new();
        for (y, line) in input.trim().split_whitespace().enumerate() {
            for (x, c) in line.trim().chars().enumerate() {
                if c == '#' {
                    prev.insert(Pos {
                        x: x as i64,
                        y: y as i64,
                    });
                }
            }
        }
        PlanetOfDiscord {
            prev,
            next: BTreeSet::default(),
        }
    }
}

impl PlanetOfDiscord {
    fn step(&self, pos: Pos) -> bool {
        let here = self.prev.contains(&pos);
        let next = Dir::all()
            .map(|dir| pos.shift(dir))
            .filter(|pos| self.prev.contains(pos))
            .count();
        match (here, next) {
        | (true, 1)
        | (false, 1)
        | (false, 2) => true,
        | _ => false,
        }
    }
}

impl Solution for PlanetOfDiscord {
    fn one(mut self) -> i64 {
        let mut seen = HashSet::new();
        loop {
            if !seen.insert(self.prev.clone()) {
                break;
            }
            self.next.clear();
            for y in 0..5 {
                for x in 0..5 {
                    let pos = Pos { x, y };
                    if self.step(pos) {
                        self.next.insert(pos);
                    }
                }
            }
            mem::swap(&mut self.prev, &mut self.next);
        }

        let mut pow = 0b1;
        let mut sum = 0;
        for y in 0..5 {
            for x in 0..5 {
                if self.prev.contains(&Pos { x, y }) {
                    sum |= pow;
                }
                pow <<= 1;
            }
        }

        sum
    }

    fn two(self) -> i64 {
        todo!()
    }
}
