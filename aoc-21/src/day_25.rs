use std::fmt;
use std::mem;

use aoc::*;

#[derive(Clone)]
pub struct SeaCucumber(Vec<Vec<Option<Cuke>>>);

#[derive(Copy, Clone, Debug)]
enum Cuke {
    E,
    S,
}

impl Fro for SeaCucumber {
    fn fro(input: &str) -> Self {
        let mut grid = Vec::new();
        for line in input.trim().split('\n') {
            let mut row = Vec::new();
            for col in line.trim().chars() {
                row.push(match col {
                    '.' => None,
                    '>' => Some(Cuke::E),
                    'v' => Some(Cuke::S),
                    _ => unreachable!(),
                });
            }
            grid.push(row);
        }
        Self(grid)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Grid([[u64; 3]; 137]);

impl Grid {
    fn new() -> Self {
        Grid([[0; 3]; 137])
    }

    fn rotate_up(mut self) -> Self {
        self.0.rotate_left(1);
        self
    }

    fn rotate_down(mut self) -> Self {
        self.0.rotate_right(1);
        self
    }

    #[rustfmt::skip]
    fn rotate_left(mut self) -> Self {
        self.0.iter_mut().for_each(|[a, b, c]| {
            *a = a.rotate_left(1);
            *b = b.rotate_left(1);
            *c = c.rotate_left(1);

            let swap = *c & 1;

            *c = *c & 0b1111_1111_1100_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000 | ((*a & 1) << 53);
            *a = *a & 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1110 | (*b & 1);
            *b = *b & 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1110 | swap;
        });

        self
    }

    #[rustfmt::skip]
    fn rotate_right(mut self) -> Self {
        self.0.iter_mut().for_each(|[a, b, c]| {
            // 139 = 64 + 64 + 11

            *a = a.rotate_right(1);
            *b = b.rotate_right(1);
            *c = c.rotate_right(1);

            let swap = *c & (1 << 52);

            *c = *c & 0b0111_1111_1110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000 | (*b & (1 << 63));
            *b = *b & 0b0111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111 | (*a & (1 << 63));
            *a = *a & 0b0111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111 | (swap << 11);

            // 63 62 61 60 | 59 58 57 56 | 55 54 53
            // .  .  .  .  | .  .  .  .  | .  .  .
            // 0  1  2  3  | 4  5  6  7  | 8  9  10
        });

        self
    }

    fn get(&self, i: usize, j: usize) -> bool {
        let hi = j / 64;
        let lo = j % 64;
        (self.0[i][hi] >> (63 - lo)) & 1 != 0
    }

    fn set(&mut self, i: usize, j: usize) {
        let hi = j / 64;
        let lo = j % 64;
        self.0[i][hi] |= 1 << (63 - lo);
    }

    fn or(mut self, other: Self) -> Self {
        self.0
            .iter_mut()
            .flatten()
            .zip(other.0.into_iter().flatten())
            .for_each(|(a, b)| *a |= b);
        self
    }

    fn and(mut self, other: Self) -> Self {
        self.0
            .iter_mut()
            .flatten()
            .zip(other.0.into_iter().flatten())
            .for_each(|(a, b)| *a &= b);
        self
    }

    #[rustfmt::skip]
    fn not(mut self) -> Self {
        self.0.iter_mut().for_each(|[a, b, c]| {
            *a = !*a;
            *b = !*b;
            *c = !*c & 0b1111_1111_1110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
        });

        self
    }

    fn is_empty(&self) -> bool {
        self.0.iter().flatten().all(|a| *a == 0)
    }
}

impl Solution for SeaCucumber {
    // fn one(mut self) -> i64 {
    //     let mut buffer = self.0.clone();

    //     let mut dirty = true;

    //     let h = self.0.len();
    //     let w = self.0[0].len();
    //     let mut steps = 0;

    //     while mem::take(&mut dirty) {
    //         steps += 1;

    //         println!("{}\n{:?}", steps, self);
    //         if steps > 5 {
    //             panic!();
    //         }

    //         for i in 0..h {
    //             for j in 0..w {
    //                 match self.0[i][j] {
    //                     None | Some(Cuke::S) => (),
    //                     Some(Cuke::E) => {
    //                         if self.0[i][(j + 1) % w].is_none() {
    //                             buffer[i][j] = None;
    //                             buffer[i][(j + 1) % w] = Some(Cuke::E);
    //                             dirty = true;
    //                         }
    //                     }
    //                 }
    //             }
    //         }

    //         for i in 0..h {
    //             for j in 0..w {
    //                 match self.0[i][j] {
    //                     None | Some(Cuke::E) => (),
    //                     Some(Cuke::S) => {
    //                         if self.0[(i + 1) % h][j].is_none() {
    //                             buffer[i][j] = None;
    //                             buffer[(i + 1) % h][j] = Some(Cuke::S);
    //                             dirty = true;
    //                         }
    //                     }
    //                 }
    //             }
    //         }

    //         mem::swap(&mut self.0, &mut buffer);
    //     }

    //     steps
    // }

    fn one(self) -> i64 {
        let mut dirty = true;

        let h = self.0.len();
        let w = self.0[0].len();

        let mut south = Grid::new();
        for i in 0..h {
            for j in 0..w {
                if let Some(Cuke::S) = self.0[i][j] {
                    south.set(i, j);
                }
            }
        }

        assert_eq!(south.rotate_down().rotate_up(), south);
        assert_eq!(south.rotate_up().rotate_down(), south);
        assert_eq!(south.rotate_left().rotate_right(), south);
        assert_eq!(south.rotate_right().rotate_left(), south);

        let mut east = Grid::new();
        for i in 0..h {
            for j in 0..w {
                if let Some(Cuke::E) = self.0[i][j] {
                    east.set(i, j);
                }
            }
        }

        let mut steps = 0;

        while mem::take(&mut dirty) {
            steps += 1;

            let closed = south.or(east);
            let open = closed.not();

            let easter = east.rotate_right();
            let next_east = easter.and(open);
            let prev_east = easter.and(closed).rotate_left();

            dirty |= !next_east.is_empty();
            east = next_east.or(prev_east);

            let closed = south.or(east);
            let open = closed.not();

            let souther = south.rotate_down();
            let next_south = souther.and(open);
            let prev_south = souther.and(closed).rotate_up();

            dirty |= !next_south.is_empty();
            south = next_south.or(prev_south);
        }

        steps
    }

    fn two(self) -> i64 {
        unreachable!()
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let h = self.0.len();
        let w = self.0[0].len();
        for i in 0..h {
            for j in 0..w {
                if self.get(i, j) {
                    write!(fmt, "X")?;
                } else {
                    write!(fmt, ".")?;
                }
            }
            writeln!(fmt)?;
        }
        Ok(())
    }
}

impl fmt::Debug for SeaCucumber {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let h = self.0.len();
        let w = self.0[0].len();
        for i in 0..h {
            for j in 0..w {
                match self.0[i][j] {
                    None => write!(fmt, ".")?,
                    Some(Cuke::E) => write!(fmt, ">")?,
                    Some(Cuke::S) => write!(fmt, "v")?,
                }
            }
            writeln!(fmt)?;
        }
        Ok(())
    }
}
