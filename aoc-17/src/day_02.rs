use std::cmp;
use std::str;
use std::usize;

#[derive(Debug)]
pub struct CorruptionChecksum {
    rows: usize, 
    cols: usize,
    grid: Vec<usize>,
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
                n.parse::<usize>()
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
    fn one(self) -> usize {
        let mut sum = 0;
        for row in 0..self.rows {
            let mut max = usize::MIN;
            let mut min = usize::MAX;
            for col in 0..self.cols {
                let n = self.grid[row * self.cols + col];
                max = cmp::max(n, max);
                min = cmp::min(n, min);
            }
            sum += max - min;
        }
        sum
    }

    fn two(self) -> usize {
        unimplemented!()
    }
}
