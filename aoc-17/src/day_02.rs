use std::cmp;
use std::str;
use std::i64;

#[derive(Debug)]
pub struct CorruptionChecksum {
    rows: usize, 
    cols: usize,
    grid: Vec<i64>,
}

impl str::FromStr for CorruptionChecksum {
    type Err = aoc::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cols = s.lines()
            .next()
            .expect("At least one row in grid")
            .trim()
            .split_whitespace()
            .count();

        let mut grid = Vec::new();

        for line in s.lines() {
            for n in line.split_whitespace() {
                n.parse::<i64>()
                    .map_err(aoc::Error::InvalidInt)
                    .map(|n| grid.push(n))?; 
            }
        }
        
        let rows = grid.len() / cols;

        Ok(CorruptionChecksum {
            rows,
            cols,
            grid,
        })
    }
}

impl aoc::Solution for CorruptionChecksum {
    fn one(self) -> i64 {
        let mut sum = 0;
        for row in 0..self.rows {
            let mut max = i64::MIN;
            let mut min = i64::MAX;
            for col in 0..self.cols {
                let n = self.grid[row * self.cols + col];
                max = cmp::max(n, max);
                min = cmp::min(n, min);
            }
            sum += max - min;
        }
        sum
    }

    fn two(self) -> i64 {
        let mut sum = 0;
        for row in 0..self.rows {
            'outer: for i in 0..self.cols {
                let a = self.grid[row * self.cols + i];
                for j in i + 1..self.cols {
                    let b = self.grid[row * self.cols + j];
                    if a % b == 0 {
                        sum += a / b;
                        break 'outer;
                    } else if b % a == 0 {
                        sum += b / a;
                        break 'outer;
                    }
                }
            }
        }
        sum
    }
}
