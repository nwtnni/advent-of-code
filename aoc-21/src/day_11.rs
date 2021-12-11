use aoc::*;

#[derive(Clone, Debug)]
pub struct DumboOctopus(Grid<i64>);

impl Fro for DumboOctopus {
    fn fro(input: &str) -> Self {
        Self(Grid::fro(input))
    }
}

impl Solution for DumboOctopus {
    fn one(mut self) -> i64 {
        let mut flashed = Vec::new();
        (0..100)
            .map(|_| step(&mut self.0, &mut flashed))
            .sum::<usize>()
            as i64
    }

    fn two(mut self) -> i64 {
        let mut flashed = Vec::new();
        let total = self.0.height() * self.0.width();
        (1..)
            .find(|_| step(&mut self.0, &mut flashed) == total)
            .unwrap()
    }
}

fn step(grid: &mut Grid<i64>, flashed: &mut Vec<(usize, usize)>) -> usize {
    let mut flashes = 0;

    for i in 0..grid.height() {
        for j in 0..grid.width() {
            match &mut grid[i][j] {
                octopus @ 9 => {
                    *octopus = 0;
                    flashes += 1;
                    flashed.push((i, j));
                }
                octopus => *octopus += 1,
            }
        }
    }

    while let Some((i, j)) = flashed.pop() {
        for (i, j) in around(grid.height(), grid.width(), i, j) {
            match &mut grid[i][j] {
                0 => (),
                octopus @ 9 => {
                    *octopus = 0;
                    flashes += 1;
                    flashed.push((i, j));
                }
                octopus => *octopus += 1,
            }
        }
    }

    flashes
}
