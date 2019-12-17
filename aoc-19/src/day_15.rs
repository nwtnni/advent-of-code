use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp;

use aoc::*;

pub struct OxygenSystem(intcode::Program);

impl Fro for OxygenSystem {
    fn fro(input: &str) -> Self {
        OxygenSystem(intcode::Program::fro(input))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Mode {
    /// Find the oxygen station.
    Find,

    /// Fill the map with oxygen.
    Fill,
}

static DIRS: [Dir; 4] = [Dir::N, Dir::S, Dir::E, Dir::W];

fn input(dir: Dir) -> i64 {
    match dir {
    | Dir::N => 1,
    | Dir::S => 2,
    | Dir::W => 3,
    | Dir::E => 4,
    }
}

impl OxygenSystem {
    fn explore(
        &mut self,
        dis: i64,
        dir: Option<Dir>,
        max: &mut i64,
        mode: Mode,
    ) -> Option<i64> {
        for next in DIRS.iter().filter(|next| Some(**next) != dir) {
            match self.0.pipe(input(*next)) {
            | Some(0) => continue,
            | Some(response) => {
                if response == 2 && mode == Mode::Find {
                    return Some(dis);
                }
                *max = cmp::max(dis, *max);
                let prev = next.flip();
                if let Some(dis) = self.explore(dis + 1, Some(prev), max, Mode::Find) {
                    return Some(dis);
                }
                self.0.pipe(input(prev));
            }
            | _ => unreachable!(),
            }
        }
        None
    }
}

impl Solution for OxygenSystem {
    fn one(mut self) -> i64 {
        let mut max = 0;
        self.explore(1, None, &mut max, Mode::Find).unwrap()
    }

    fn two(mut self) -> i64 {
        let mut max = 0;
        self.explore(1, None, &mut 0, Mode::Find);
        self.explore(1, None, &mut max, Mode::Fill);
        max
    }
}
