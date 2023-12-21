use std::collections::HashMap;

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

    fn two(mut self) -> i64 {
        todo!();
    }
}
