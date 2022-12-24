use std::{cmp, collections::VecDeque, fmt};

use aoc::*;
use priority_queue::PriorityQueue;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct BlizzardBasin {
    width: usize,
    north: Vec<u128>,
    south: Vec<u128>,
    east: Vec<u128>,
    west: Vec<u128>,
}

impl Fro for BlizzardBasin {
    fn fro(input: &str) -> Self {
        let mut north = Vec::new();
        let mut south = Vec::new();
        let mut east = Vec::new();
        let mut west = Vec::new();
        let mut width = 0;

        input
            .trim()
            .split('\n')
            .skip(1)
            .take_while(|line| !line.starts_with("##"))
            .for_each(|line| {
                let mut n = 0;
                let mut s = 0;
                let mut e = 0;
                let mut w = 0;

                for (i, char) in line.chars().skip(1).take(line.len() - 2).enumerate() {
                    let modify = match char {
                        '^' => Some(&mut n),
                        'v' => Some(&mut s),
                        '>' => Some(&mut e),
                        '<' => Some(&mut w),
                        _ => None,
                    };

                    if let Some(modify) = modify {
                        *modify |= 1 << i;
                    }

                    width = i + 1;
                }

                north.push(n);
                south.push(s);
                east.push(e);
                west.push(w);
            });

        Self {
            width,
            north,
            south,
            east,
            west,
        }
    }
}

impl fmt::Display for BlizzardBasin {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for j in 0..self.width + 2 {
            write!(fmt, "{}", if j == 1 { '.' } else { '#' })?;
        }

        writeln!(fmt)?;

        for i in 0..self.north.len() {
            write!(fmt, "#")?;

            for j in 0..self.width {
                let north = self.north[i] & (1 << j) > 0;
                let south = self.south[i] & (1 << j) > 0;
                let east = self.east[i] & (1 << j) > 0;
                let west = self.west[i] & (1 << j) > 0;

                let char = match north as u8 + south as u8 + east as u8 + west as u8 {
                    1 if north => '^',
                    1 if south => 'v',
                    1 if east => '>',
                    1 if west => '<',
                    0 => '.',
                    2 => '2',
                    3 => '3',
                    4 => '4',
                    _ => unreachable!(),
                };

                write!(fmt, "{}", char)?;
            }

            writeln!(fmt, "#")?;
        }

        for j in 0..self.width + 2 {
            write!(fmt, "{}", if j == self.width { '.' } else { '#' })?;
        }

        Ok(())
    }
}

impl BlizzardBasin {
    fn step(&mut self) {
        let mask = (1 << self.width) - 1;
        self.north.rotate_left(1);
        self.south.rotate_right(1);
        self.west
            .iter_mut()
            .for_each(|row| *row = mask & ((*row >> 1) | (*row << (self.width - 1))));
        self.east
            .iter_mut()
            .for_each(|row| *row = mask & ((*row << 1) | (*row >> (self.width - 1))));
    }

    fn reachable(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)]
            .into_iter()
            .map(move |(dx, dy)| Pos {
                x: pos.x + dx,
                y: pos.y + dy,
            })
            .filter(|pos| self.bounded(*pos))
            .filter(|pos| !self.occupied(*pos))
    }

    fn occupied(&self, pos: Pos) -> bool {
        let x = pos.x;
        let y = pos.y as usize;
        (self.north[y] | self.south[y] | self.east[y] | self.west[y]) & (1 << x) > 0
    }

    fn bounded(&self, pos: Pos) -> bool {
        pos.x >= 0 && pos.x < self.width as i64 && pos.y >= 0 && pos.y < self.north.len() as i64
    }
}

impl Solution for BlizzardBasin {
    fn one(mut self) -> i64 {
        let mut queue = PriorityQueue::new();

        let start = Pos { x: 0, y: 0 };
        let finish = Pos {
            x: self.width as i64 - 1,
            y: self.north.len() as i64 - 1,
        };

        self.step();
        println!("{}", self);

        queue.push((start, self), cmp::Reverse(2));

        while let Some(((pos, mut state), cmp::Reverse(distance))) = queue.pop() {
            if pos == finish {
                return distance;
            }

            state.step();

            for next in state.reachable(pos) {
                queue.push_increase((next, state.clone()), cmp::Reverse(distance + 1));
            }
        }

        unreachable!()
    }

    fn two(self) -> i64 {
        todo!()
    }
}
