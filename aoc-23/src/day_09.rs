use aoc::*;

#[derive(Clone, Debug)]
pub struct MirageMaintenance(Vec<Vec<i64>>);

impl Fro for MirageMaintenance {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| line.split_whitespace().map(i64::fro).collect())
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for MirageMaintenance {
    fn one(self) -> i64 {
        self.0
            .iter()
            .map(|line| {
                let mut grid = vec![line.clone()];

                while grid.last().unwrap().iter().any(|element| *element != 0) {
                    let next = grid
                        .last()
                        .unwrap()
                        .windows(2)
                        .map(|window| window[1] - window[0])
                        .collect::<Vec<_>>();
                    grid.push(next);
                }

                grid.last_mut().unwrap().push(0);

                for i in (0..grid.len()).rev().skip(1) {
                    let a = *grid[i + 1].last().unwrap();
                    let b = *grid[i].last().unwrap();
                    grid[i].push(a + b);
                }

                *grid[0].last().unwrap()
            })
            .sum()
    }

    fn two(self) -> i64 {
        self.0
            .iter()
            .map(|line| {
                let mut grid = vec![line.clone()];

                while grid.last().unwrap().iter().any(|element| *element != 0) {
                    let next = grid
                        .last()
                        .unwrap()
                        .windows(2)
                        .map(|window| window[1] - window[0])
                        .collect::<Vec<_>>();
                    grid.push(next);
                }

                grid.last_mut().unwrap().insert(0, 0);

                for i in (0..grid.len()).rev().skip(1) {
                    let a = *grid[i + 1].first().unwrap();
                    let b = *grid[i].first().unwrap();
                    grid[i].insert(0, b - a);
                }

                *grid[0].first().unwrap()
            })
            .sum()
    }
}
