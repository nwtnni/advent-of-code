use std::collections::HashSet;
use std::collections::HashMap;
use std::str;

#[derive(Clone, Debug)]
pub struct CrossedWires(Vec<Wire>, Vec<Wire>);

#[derive(Copy, Clone, Debug)]
struct Wire {
    dir: aoc::Dir,
    len: i32,
}

impl str::FromStr for Wire {
    type Err = aoc::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iter = input.chars();
        let dir = match iter.next() {
            | Some('R') => aoc::Dir::E,
            | Some('D') => aoc::Dir::S,
            | Some('U') => aoc::Dir::N,
            | Some('L') => aoc::Dir::W,
            | _ => unreachable!(),
        };
        let len = iter
            .as_str()
            .parse::<i32>()
            .map_err(aoc::Error::InvalidInt)?;
        Ok(Wire {
            dir,
            len,
        })
    }
}

impl str::FromStr for CrossedWires {
    type Err = aoc::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut wires = input.split_whitespace();
        let l = wires.next()
            .unwrap()
            .split(',')
            .filter_map(|wire| wire.parse::<Wire>().ok())
            .collect();
        let r = wires.next()
            .unwrap()
            .split(',')
            .filter_map(|wire| wire.parse::<Wire>().ok())
            .collect();
        Ok(CrossedWires(l, r))
    }
}

impl aoc::Solution for CrossedWires {
    fn one(self) -> i32 {
        let mut l_seen = HashSet::new();
        let mut p = aoc::Pos::default();
        for wire in &self.0 {
            for _ in 0..wire.len {
                p.shift_mut(wire.dir);
                l_seen.insert(p);
            }
        }

        let mut r_seen = HashSet::new();
        let mut p = aoc::Pos::default();
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

    fn two(self) -> i32 {
        let mut l_dist = HashMap::new();
        let mut l_seen = HashSet::new();
        let mut p = aoc::Pos::default();
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
        let mut p = aoc::Pos::default();
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
