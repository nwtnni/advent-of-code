use std::collections::HashSet;
use std::collections::HashMap;
use std::str;

use aoc::*;

#[derive(Clone, Debug)]
pub struct CrossedWires(Vec<Wire>, Vec<Wire>);

#[derive(Copy, Clone, Debug)]
struct Wire {
    dir: Dir,
    len: i64,
}

impl Fro for Wire {
    fn fro(input: &str) -> Self {
        let mut iter = input.chars();
        let dir = match iter.give() {
        | 'R' => Dir::E,
        | 'D' => Dir::S,
        | 'U' => Dir::N,
        | 'L' => Dir::W,
        | _ => unreachable!(),
        };
        let len = iter
            .as_str()
            .to::<i64>();
        Wire {
            dir,
            len,
        }
    }
}

impl str::FromStr for CrossedWires {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut wires = input.split_whitespace();
        let l = wires.give()
            .split(',')
            .map(Wire::fro)
            .collect();
        let r = wires.give()
            .split(',')
            .map(Wire::fro)
            .collect();
        Ok(CrossedWires(l, r))
    }
}

impl Solution for CrossedWires {
    fn one(self) -> i64 {
        let mut l_seen = HashSet::new();
        let mut p = Pos::default();
        for wire in &self.0 {
            for _ in 0..wire.len {
                p.shift_mut(wire.dir);
                l_seen.insert(p);
            }
        }

        let mut r_seen = HashSet::new();
        let mut p = Pos::default();
        for wire in &self.1 {
            for _ in 0..wire.len {
                p.shift_mut(wire.dir);
                r_seen.insert(p);
            }
        }

        l_seen.intersection(&r_seen)
            .map(|p| p.x.abs() + p.y.abs())
            .min()
            .unwrap()
    }

    fn two(self) -> i64 {
        let mut l_dist = HashMap::new();
        let mut l_seen = HashSet::new();
        let mut p = Pos::default();
        let mut s = 0;
        for wire in &self.0 {
            for _ in 0..wire.len {
                p.shift_mut(wire.dir);
                s += 1;
                l_seen.insert(p);
                l_dist.entry(p).or_insert(s);
            }
        }

        let mut r_dist = HashMap::new();
        let mut r_seen = HashSet::new();
        let mut p = Pos::default();
        let mut s = 0;
        for wire in &self.1 {
            for _ in 0..wire.len {
                p.shift_mut(wire.dir);
                s += 1;
                r_seen.insert(p);
                r_dist.entry(p).or_insert(s);
            }
        }

        l_seen.intersection(&r_seen)
            .map(|p| l_dist[p] + r_dist[p])
            .min()
            .unwrap()
    }
}
