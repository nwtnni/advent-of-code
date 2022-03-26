use std::collections::HashSet;
use std::ops;

use aoc::*;

#[derive(Clone, Debug)]
pub struct TobogganTrajectory {
    rows: usize,
    cols: usize,
    grid: HashSet<Pos>,
}

impl ops::Deref for TobogganTrajectory {
    type Target = HashSet<Pos>;
    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl ops::DerefMut for TobogganTrajectory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

// Grid-based parsing
impl Fro for TobogganTrajectory {
    fn fro(input: &str) -> Self {
        let mut grid = HashSet::new();
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

                if col == '#' {
                    grid.insert(pos);
                }
            }
        }
        Self { rows, cols, grid }
    }
}

impl Solution for TobogganTrajectory {
    fn one(self) -> i64 {
        (0..self.rows)
            .into_iter()
            .filter(|row| {
                let row = *row as i64;
                self.contains(&Pos {
                    x: (row * 3) % (self.cols as i64),
                    y: row,
                })
            })
            .count() as i64
    }

    fn two(self) -> i64 {
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|(dx, dy)| {
                (0..self.rows)
                    .step_by(*dy as usize)
                    .into_iter()
                    .filter(|row| {
                        let row = *row as i64;
                        self.contains(&Pos {
                            x: ((row / dy) * dx) % (self.cols as i64),
                            y: row,
                        })
                    })
                    .count()
            })
            .product::<usize>() as i64
    }
}

#[cfg(test)]
mod tests {

    use aoc::Fro as _;
    use aoc::Solution as _;

    static EXAMPLE: &str = "\
        ..##.......\n\
        #...#...#..\n\
        .#....#..#.\n\
        ..#.#...#.#\n\
        .#...##..#.\n\
        ..#.##.....\n\
        .#.#.#....#\n\
        .#........#\n\
        #.##...#...\n\
        #...##....#\n\
        .#..#...#.#\
    ";

    #[test]
    fn part_one() {
        assert_eq!(super::TobogganTrajectory::fro(EXAMPLE).one(), 7);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::TobogganTrajectory::fro(EXAMPLE).two(), 336);
    }
}
