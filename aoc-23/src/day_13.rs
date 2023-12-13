use aoc::*;

#[derive(Clone, Debug)]
pub struct PointOfIncidence(Vec<Grid>);

type Grid = Vec<Vec<bool>>;

impl Fro for PointOfIncidence {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split("\n\n")
            .map(|grid| {
                grid.split('\n')
                    .map(|line| line.chars().map(|char| char == '#').collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

fn horizontal(grid: &[Vec<bool>]) -> Vec<usize> {
    let mut all = Vec::new();
    for i in 1..grid.len() {
        let before = 0..i;
        let after = i..grid.len();

        if grid[before]
            .iter()
            .rev()
            .zip(&grid[after])
            .all(|(l, r)| l == r)
        {
            all.push(i);
        }
    }

    all
}

fn vertical(grid: &[Vec<bool>]) -> Vec<usize> {
    let mut all = Vec::new();
    for j in 1..grid[0].len() {
        let before = 0..j;
        let after = j..grid[0].len();

        if before.rev().zip(after).all(|(l, r)| {
            grid.iter().map(|row| row[l]).collect::<Vec<_>>()
                == grid.iter().map(|row| row[r]).collect::<Vec<_>>()
        }) {
            all.push(j);
        }
    }

    all
}

impl Solution for PointOfIncidence {
    fn one(self) -> i64 {
        self.0
            .iter()
            .map(|grid| {
                horizontal(grid)
                    .first()
                    .map(|h| h * 100)
                    .unwrap_or_else(|| vertical(grid)[0])
            })
            .sum::<usize>() as i64
    }

    fn two(mut self) -> i64 {
        self.0.iter_mut().map(mutate).sum::<usize>() as i64
    }
}

fn mutate(grid: &mut Vec<Vec<bool>>) -> usize {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    enum Line {
        H(usize),
        V(usize),
    }

    let old = horizontal(grid)
        .first()
        .copied()
        .map(Line::H)
        .unwrap_or_else(|| vertical(grid)[0].tap(Line::V));

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            grid[i][j] ^= true;
            for h in horizontal(grid) {
                if old != Line::H(h) {
                    return h * 100;
                }
            }
            for v in vertical(grid) {
                if old != Line::V(v) {
                    return v;
                }
            }
            grid[i][j] ^= true;
        }
    }

    unreachable!()
}
