use std::cmp;
use std::collections::VecDeque;

use aoc::*;

#[derive(Clone, Debug)]
pub struct HillClimbingAlgorithm {
    start: (usize, usize),
    finish: (usize, usize),
    grid: Vec<Vec<u8>>,
}

impl Fro for HillClimbingAlgorithm {
    fn fro(input: &str) -> Self {
        let mut grid = input
            .trim()
            .split('\n')
            .map(|line| line.bytes().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut start = (0, 0);
        let mut finish = (0, 0);

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == b'S' {
                    grid[i][j] = b'a';
                    start = (i, j);
                }

                if grid[i][j] == b'E' {
                    grid[i][j] = b'z';
                    finish = (i, j);
                }
            }
        }

        Self {
            start,
            finish,
            grid,
        }
    }
}

impl Solution for HillClimbingAlgorithm {
    fn one(self) -> i64 {
        let distance = self.flood();
        distance[self.start.0][self.start.1]
    }

    fn two(self) -> i64 {
        let distance = self.flood();

        let mut min = i64::MAX;

        #[allow(clippy::needless_range_loop)]
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if self.grid[i][j] == b'a' {
                    min = cmp::min(distance[i][j], min);
                }
            }
        }

        min
    }
}

impl HillClimbingAlgorithm {
    fn flood(&self) -> Vec<Vec<i64>> {
        let mut flood = VecDeque::new();
        let mut distance = vec![vec![i64::MAX; self.grid[0].len()]; self.grid.len()];

        flood.push_back(self.finish);
        distance[self.finish.0][self.finish.1] = 0;

        while let Some((i, j)) = flood.pop_front() {
            for (y, x) in [
                if i < self.grid.len() - 1 {
                    Some((i + 1, j))
                } else {
                    None
                },
                if i > 0 { Some((i - 1, j)) } else { None },
                if j < self.grid[0].len() - 1 {
                    Some((i, j + 1))
                } else {
                    None
                },
                if j > 0 { Some((i, j - 1)) } else { None },
            ]
            .into_iter()
            .flatten()
            {
                if self.grid[y][x] + 1 >= self.grid[i][j]
                    && distance[i][j].saturating_add(1) < distance[y][x]
                {
                    distance[y][x] = distance[i][j].saturating_add(1);
                    flood.push_back((y, x));
                }
            }
        }

        distance
    }
}
