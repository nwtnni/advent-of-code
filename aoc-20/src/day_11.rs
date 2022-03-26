use std::mem;

use aoc::*;

#[derive(Clone, Debug)]
pub struct SeatingSystem {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<Seat>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Seat {
    Empty,
    Floor,
    Occupied,
}

impl Fro for SeatingSystem {
    fn fro(input: &str) -> Self {
        let grid = input
            .trim()
            .split('\n')
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|char| match char {
                        '#' => Seat::Occupied,
                        'L' => Seat::Empty,
                        '.' => Seat::Floor,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let rows = grid.len();
        let cols = grid[0].len();
        Self { rows, cols, grid }
    }
}

impl SeatingSystem {
    fn get(&self, row: i64, col: i64) -> Option<Seat> {
        self.grid
            .get(row as usize)
            .and_then(|row| row.get(col as usize))
            .copied()
    }
}

impl Solution for SeatingSystem {
    fn one(mut self) -> i64 {
        let mut grid = self.grid.clone();

        loop {
            for r in 0..self.rows as i64 {
                for c in 0..self.cols as i64 {
                    let mut around = 0;

                    for dr in &[-1, 0, 1] {
                        for dc in &[-1, 0, 1] {
                            if *dr == 0i64 && *dc == 0i64 {
                                continue;
                            }

                            if let Some(Seat::Occupied) = self.get(r + dr, c + dc) {
                                around += 1;
                            }
                        }
                    }

                    grid[r as usize][c as usize] = match (self.get(r, c), around) {
                        (Some(Seat::Empty), 0) => Seat::Occupied,
                        (Some(Seat::Occupied), 4..=i64::MAX) => Seat::Empty,
                        (Some(seat), _) => seat,
                        _ => unreachable!(),
                    }
                }
            }

            if grid == self.grid {
                break self
                    .grid
                    .iter()
                    .flatten()
                    .filter(|seat| **seat == Seat::Occupied)
                    .count() as i64;
            }

            mem::swap(&mut grid, &mut self.grid);
        }
    }

    fn two(mut self) -> i64 {
        let mut grid = self.grid.clone();

        loop {
            for r in 0..self.rows as i64 {
                for c in 0..self.cols as i64 {
                    let mut around = 0;

                    for dr in &[-1, 0, 1] {
                        for dc in &[-1, 0, 1] {
                            if *dr == 0i64 && *dc == 0i64 {
                                continue;
                            }

                            for s in 1.. {
                                match self.get(r + s * dr, c + s * dc) {
                                    None => break,
                                    Some(Seat::Floor) => continue,
                                    Some(Seat::Empty) => break,
                                    Some(Seat::Occupied) => {
                                        around += 1;
                                        // Here lies the `break` statement which cost me
                                        // half an hour and a lot of frustration :(
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    grid[r as usize][c as usize] = match (self.get(r, c), around) {
                        (Some(Seat::Empty), 0) => Seat::Occupied,
                        (Some(Seat::Occupied), 5..=i64::MAX) => Seat::Empty,
                        (Some(seat), _) => seat,
                        _ => unreachable!(),
                    }
                }
            }

            if grid == self.grid {
                break self
                    .grid
                    .iter()
                    .flatten()
                    .filter(|seat| **seat == Seat::Occupied)
                    .count() as i64;
            }

            mem::swap(&mut grid, &mut self.grid);
        }
    }
}
