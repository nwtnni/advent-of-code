use std::{cmp, collections::HashMap};

use aoc::*;

#[derive(Clone, Debug)]
pub struct TreetopTreeHouse {
    cols: usize,
    rows: usize,
    grid: HashMap<Pos, i64>,
}

#[derive(Copy, Clone, Debug)]
struct Visible {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
}

impl Visible {
    fn any(&self) -> bool {
        self.left || self.right || self.top || self.bottom
    }
}

impl Default for Visible {
    fn default() -> Self {
        Self {
            left: true,
            right: true,
            top: true,
            bottom: true,
        }
    }
}

impl Fro for TreetopTreeHouse {
    fn fro(input: &str) -> Self {
        let mut grid = HashMap::new();
        let mut rows = 0;
        let mut cols = 0;
        for (y, row) in input.trim().split('\n').enumerate() {
            rows += 1;
            cols = 0;
            for (x, col) in row.trim().chars().enumerate() {
                cols += 1;
                let pos = Pos {
                    x: x as i64,
                    y: y as i64,
                };

                let cell = (col as u8 - b'0') as i64;

                grid.insert(pos, cell);
            }
        }

        Self { rows, cols, grid }
    }
}

impl Solution for TreetopTreeHouse {
    fn one(self) -> i64 {
        let mut visiblity = HashMap::<_, Visible>::new();

        for i in 1..self.rows - 1 {
            let mut max_row = self.grid[&Pos { y: i as i64, x: 0 }];

            for j in 1..self.cols - 1 {
                let pos = Pos {
                    y: i as i64,
                    x: j as i64,
                };

                let visible = self.grid[&pos] > max_row;
                max_row = cmp::max(max_row, self.grid[&pos]);

                visiblity.entry(pos).or_default().left &= visible;
            }

            max_row = self.grid[&Pos {
                y: i as i64,
                x: self.cols as i64 - 1,
            }];

            for j in (1..self.cols - 1).rev() {
                let pos = Pos {
                    y: i as i64,
                    x: j as i64,
                };

                let visible = self.grid[&pos] > max_row;
                max_row = cmp::max(max_row, self.grid[&pos]);

                visiblity.entry(pos).or_default().right &= visible;
            }
        }

        for j in 1..self.cols - 1 {
            let mut max_col = self.grid[&Pos { y: 0, x: j as i64 }];

            for i in 1..self.rows - 1 {
                let pos = Pos {
                    y: i as i64,
                    x: j as i64,
                };

                let visible = self.grid[&pos] > max_col;
                max_col = cmp::max(max_col, self.grid[&pos]);

                visiblity.entry(pos).or_default().top &= visible;
            }

            max_col = self.grid[&Pos {
                y: self.rows as i64 - 1,
                x: j as i64,
            }];

            for i in (1..self.rows - 1).rev() {
                let pos = Pos {
                    y: i as i64,
                    x: j as i64,
                };

                let visible = self.grid[&pos] > max_col;
                max_col = cmp::max(max_col, self.grid[&pos]);

                visiblity.entry(pos).or_default().bottom &= visible;
            }
        }

        self.grid
            .keys()
            .filter(|pos| visiblity.entry(**pos).or_default().any())
            .count() as i64
    }

    fn two(self) -> i64 {
        let mut max = 0;

        for i in 1..self.rows - 1 {
            for j in 1..self.cols - 1 {
                let center = Pos {
                    y: i as i64,
                    x: j as i64,
                };

                let score = Dir::all()
                    .map(|dir| {
                        let mut stop = false;
                        (1..)
                            .take_while(|dj| {
                                let pos = center.shiftn(dir, *dj);
                                match self.grid.get(&pos).copied() {
                                    None => false,
                                    Some(_) if stop => false,
                                    Some(height) => {
                                        stop = height >= self.grid[&center];
                                        true
                                    }
                                }
                            })
                            .count() as i64
                    })
                    .product();

                max = cmp::max(max, score);
            }
        }

        max
    }
}
