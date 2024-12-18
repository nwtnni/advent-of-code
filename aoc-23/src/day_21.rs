use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct StepCounter {
    start: Pos,
    rocks: Grid,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Grid {
    rows: usize,
    cols: usize,
    size: usize,
    bits: Vec<u64>,
}

const BITS: usize = 64;

impl Fro for StepCounter {
    fn fro(input: &str) -> Self {
        let rows = input.lines().count();
        let cols = input.lines().give().len();
        let size = (cols + BITS - 1) / BITS;
        let mut bits = vec![0; rows * size];
        let mut start = Pos::default();

        for (i, line) in input.trim().split('\n').enumerate() {
            let word = i * size;

            for (j, c) in line.chars().enumerate() {
                match c {
                    'S' => start = Pos::from_i_j(i as i64, j as i64),
                    '#' => bits[word + (j / BITS)] |= 1 << (j % BITS),
                    _ => (),
                }
            }
        }

        Self {
            start,
            rocks: Grid {
                rows,
                cols,
                size,
                bits,
            },
        }
    }
}

impl Grid {
    fn clear(&mut self) {
        self.bits.iter_mut().for_each(|word| *word = 0)
    }

    #[allow(dead_code)]
    fn get(&self, pos: Pos) -> Option<bool> {
        let (index, bit) = self.index(pos)?;
        Some(self.bits[index] & (1 << bit) > 0)
    }

    fn set(&mut self, pos: Pos, value: bool) {
        let (index, bit) = self.index(pos).unwrap();
        if value {
            self.bits[index] |= 1 << bit;
        } else {
            self.bits[index] &= !(1 << bit);
        }
    }

    fn index(&self, pos: Pos) -> Option<(usize, usize)> {
        let i = match pos.i() {
            i if i < 0 || i >= self.rows as i64 => return None,
            i => i as usize,
        };

        let j = match pos.j() {
            j if j < 0 || j >= self.cols as i64 => return None,
            j => j as usize,
        };

        Some((i * self.size + j / BITS, j % BITS))
    }

    fn or(&mut self, other: &Self) {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        std::iter::zip(&mut self.bits, &other.bits).for_each(|(l, r)| *l |= r);
    }

    fn and(&mut self, other: &Self) {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        std::iter::zip(&mut self.bits, &other.bits).for_each(|(l, r)| *l &= r);
    }

    fn not(&mut self) {
        self.bits.iter_mut().for_each(|word| *word = !*word)
    }

    fn shift(&mut self, dir: Dir) {
        const HI: u64 = 1 << (BITS - 1);
        const LO: u64 = 1;

        match dir {
            Dir::N => {
                self.bits.rotate_left(self.size);
                self.bits
                    .iter_mut()
                    .rev()
                    .take(self.size)
                    .for_each(|word| *word = 0);
            }
            Dir::S => {
                self.bits.rotate_right(self.size);
                self.bits
                    .iter_mut()
                    .take(self.size)
                    .for_each(|word| *word = 0);
            }
            Dir::W => {
                for row in 0..self.rows {
                    let base = row * self.size;
                    let offset = self.size - 1;

                    let word = &mut self.bits[base + offset];
                    *word = word.rotate_right(1);
                    let mut carry = *word & HI;
                    *word &= !HI;

                    for offset in (0..self.size - 1).rev() {
                        let save = carry;
                        let word = &mut self.bits[base + offset];
                        *word = word.rotate_right(1);
                        carry = *word & HI;
                        *word = *word & !HI | save;
                    }
                }
            }
            Dir::E => {
                let last = self.cols % 64;

                for row in 0..self.rows {
                    let base = row * self.size;
                    let mut carry = 0;

                    for offset in 0..self.size - 1 {
                        let save = carry;
                        let word = &mut self.bits[base + offset];
                        *word = word.rotate_left(1);
                        carry = *word & LO;
                        *word = *word & !LO | save;
                    }

                    let offset = self.size - 1;
                    let word = &mut self.bits[base + offset];
                    *word = word.rotate_left(1);
                    *word = *word & !(1 << last) | carry;
                }
            }
        }
    }

    fn len(&self) -> u32 {
        self.bits.iter().map(|word| word.count_ones()).sum()
    }

    #[allow(dead_code)]
    fn debug(&self, print: char) {
        for i in 0..self.rows {
            if i > 1 {
                return;
            }
            for j in 0..self.cols {
                if self.get(Pos::from_i_j(i as i64, j as i64)).unwrap() {
                    print!("{}", print);
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

impl Solution for StepCounter {
    fn one(mut self) -> i64 {
        // dbg!(self.rocks.cols);
        // for dir in Dir::all() {
        //     dbg!(dir);
        //     // if matches!(dir, Dir::E) {
        //     //     continue;
        //     // }
        //     let mut after = self.rocks.clone();
        //     after.shift(dir);
        //     after.debug('#');
        //     after.shift(dir.flip());
        //     println!();
        //     after.debug('#');
        //     for i in 0..self.rocks.rows {
        //         for j in 0..self.rocks.cols {
        //             if self.rocks.get(Pos::from_i_j(i as i64, j as i64)).unwrap()
        //                 != after.get(Pos::from_i_j(i as i64, j as i64)).unwrap()
        //             {
        //                 panic!("{}, {}", i, j);
        //             }
        //         }
        //     }
        //     assert_eq!(self.rocks, after);
        // }

        let mut start = self.rocks.clone();
        start.clear();
        start.set(self.start, true);

        self.rocks.not();

        for _ in 0..64 {
            let mut next = start.clone();
            next.clear();

            Dir::all()
                .map(|dir| start.clone().tap_mut(|grid| grid.shift(dir)))
                .for_each(|step| next.or(&step));

            next.and(&self.rocks);

            start = next;
        }

        start.len() as i64
    }

    fn two(self) -> i64 {
        let mut start = ProvenanceGrid(vec![vec![Vec::new(); self.rocks.cols]; self.rocks.rows]);

        start.0[self.start.i() as usize][self.start.j() as usize].push((0, 0));

        let mut garbage = HashSet::new();

        for i in 0..26501365 {
            dbg!(i);

            if i % 131 == 0 {
                start.gc(&mut garbage);
                dbg!(&garbage);
            }

            start.step(&garbage);
        }

        start.gc(&mut garbage);
        let chunk = (self.rocks.rows * self.rocks.cols) - self.rocks.len() as usize;

        (garbage.len() * chunk + start.0.iter().flatten().flatten().count()) as i64
    }
}

#[derive(Clone)]
struct ProvenanceGrid(Vec<Vec<Vec<(i32, i32)>>>);

impl ProvenanceGrid {
    fn step(&mut self, garbage: &HashSet<(i32, i32)>) {
        for (i, dir) in Dir::all().enumerate() {
            let mut next = self.clone();
            next.shift(dir);

            match dir {
                Dir::N => next.0.last_mut().unwrap().iter_mut().for_each(|set| {
                    set.retain_mut(|(i, j)| {
                        *i -= 1;
                        !garbage.contains(&(*i, *j))
                    })
                }),
                Dir::S => next.0.last_mut().unwrap().iter_mut().for_each(|set| {
                    set.retain_mut(|(i, j)| {
                        *i += 1;
                        !garbage.contains(&(*i, *j))
                    })
                }),
                Dir::W => next
                    .0
                    .iter_mut()
                    .filter_map(|row| row.last_mut())
                    .for_each(|set| {
                        set.retain_mut(|(i, j)| {
                            *j -= 1;
                            !garbage.contains(&(*i, *j))
                        })
                    }),
                Dir::E => next
                    .0
                    .iter_mut()
                    .filter_map(|row| row.first_mut())
                    .for_each(|set| {
                        set.retain_mut(|(i, j)| {
                            *j += 1;
                            !garbage.contains(&(*i, *j))
                        })
                    }),
            }

            if i == 0 {
                self.clear();
            }

            self.union(next);
        }
    }

    fn clear(&mut self) {
        self.0.iter_mut().flatten().for_each(|set| set.clear());
    }

    fn shift(&mut self, dir: Dir) {
        match dir {
            Dir::N => self.0.rotate_right(1),
            Dir::S => self.0.rotate_left(1),
            Dir::W => self.0.iter_mut().for_each(|row| row.rotate_left(1)),
            Dir::E => self.0.iter_mut().for_each(|row| row.rotate_right(1)),
        }
    }

    fn union(&mut self, other: Self) {
        for (ls, rs) in self
            .0
            .iter_mut()
            .flatten()
            .zip(other.0.into_iter().flatten())
        {
            for r in rs {
                if !ls.contains(&r) {
                    ls.push(r);
                }
            }
        }
    }

    fn gc(&mut self, garbage: &mut HashSet<(i32, i32)>) {
        let mut sets = self.0.iter().flatten();

        let mut intersection = sets.give().iter().copied().collect::<HashSet<_>>();
        for set in sets {
            intersection.retain(|chunk| set.contains(chunk));
        }

        for chunk in &intersection {
            garbage.insert(*chunk);
        }

        for set in self.0.iter_mut().flatten() {
            set.retain(|chunk| !intersection.contains(chunk));
        }
    }
}
