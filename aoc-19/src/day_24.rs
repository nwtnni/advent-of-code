use std::cmp;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter;
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
        matches!((here, next), (true, 1) | (false, 1) | (false, 2))
    }
}

fn shift(pos: Pos, dir: Dir, depth: i64) -> (i64, impl Iterator<Item = Pos>) {
    let pos = pos.shift(dir);
    if pos.x < 0 {
        (depth - 1, Or::L(iter::once(pos!(1, 2))))
    } else if pos.x >= 5 {
        (depth - 1, Or::L(iter::once(pos!(3, 2))))
    } else if pos.y < 0 {
        (depth - 1, Or::L(iter::once(pos!(2, 1))))
    } else if pos.y >= 5 {
        (depth - 1, Or::L(iter::once(pos!(2, 3))))
    } else if pos.x == 2 && pos.y == 2 {
        macro_rules! iter {
            ($array:expr) => {
                (depth + 1, Or::R($array.iter().copied()))
            };
        }
        match dir {
            Dir::N => iter!([pos!(0, 4), pos!(1, 4), pos!(2, 4), pos!(3, 4), pos!(4, 4)]),
            Dir::S => iter!([pos!(0, 0), pos!(1, 0), pos!(2, 0), pos!(3, 0), pos!(4, 0)]),
            Dir::E => iter!([pos!(0, 0), pos!(0, 1), pos!(0, 2), pos!(0, 3), pos!(0, 4)]),
            Dir::W => iter!([pos!(4, 0), pos!(4, 1), pos!(4, 2), pos!(4, 3), pos!(4, 4)]),
        }
    } else {
        (depth, Or::L(iter::once(pos)))
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
        let mut next = HashMap::<i64, BTreeSet<Pos>>::new();
        let mut prev = HashMap::new();
        prev.insert(0, self.prev);

        for _ in 0..200 {
            next.clear();

            let (min_depth, max_depth) = prev
                .keys()
                .fold((std::i64::MAX, std::i64::MIN), |(min, max), &depth| {
                    (cmp::min(min, depth), cmp::max(max, depth))
                });

            for depth in min_depth - 1..=max_depth + 1 {
                for y in 0..5 {
                    for x in 0..5 {
                        // Skip middle recursive hole
                        if x == 2 && y == 2 {
                            continue;
                        }

                        let pos = Pos { x, y };
                        let here = prev
                            .get(&depth)
                            .map(|bugs| bugs.contains(&pos))
                            .unwrap_or(false);

                        let mut around = 0;

                        for dir in Dir::all() {
                            let (depth, iter) = shift(pos, dir, depth);
                            if let Some(bugs) = prev.get(&depth) {
                                for pos in iter {
                                    if bugs.contains(&pos) {
                                        around += 1;
                                    }
                                }
                            }
                        }

                        if matches!((here, around), (true, 1) | (false, 1) | (false, 2)) {
                            next.entry(depth).or_default().insert(pos);
                        }
                    }
                }
            }

            mem::swap(&mut prev, &mut next);
        }

        prev.values()
            .map(|bugs| bugs.len())
            .map(|len| len as i64)
            .sum()
    }
}
