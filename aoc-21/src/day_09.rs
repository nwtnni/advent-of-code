use aoc::*;

#[derive(Clone, Debug)]
pub struct SmokeBasin(Vec<Vec<i64>>);

impl Fro for SmokeBasin {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                line.chars().map(|char| (char as u8 - b'0') as i64).collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for SmokeBasin {
    fn one(self) -> i64 {
        minima(&self.0)
            .into_iter()
            .map(|(i, j)| self.0[i][j] + 1)
            .sum()
    }

    fn two(mut self) -> i64 {
        let minima = minima(&self.0);
        let mut basins = Vec::new();

        for point in minima {
            let mut size = 0;
            flood(point, &mut size, &mut self.0);
            basins.push(size);
        }

        basins
            .tap_mut(|basins| basins.sort())
            .into_iter()
            .rev()
            .take(3)
            .product()
    }
}

fn minima(grid: &[Vec<i64>]) -> Vec<(usize, usize)> {
    let h = grid.len();
    let w = grid[0].len();
    let mut minima = Vec::new();

    for i in 0..h {
        for j in 0..w {
            let height = grid[i][j];

            if let Some(&up) = grid.get(i.wrapping_sub(1)).and_then(|row| row.get(j)) {
                if up <= height {
                    continue;
                }
            }

            if let Some(&down) = grid.get(i + 1).and_then(|row| row.get(j)) {
                if down <= height {
                    continue;
                }
            }

            if let Some(&left) = grid.get(i).and_then(|row| row.get(j.wrapping_sub(1))) {
                if left <= height {
                    continue;
                }
            }

            if let Some(&right) = grid.get(i).and_then(|row| row.get(j + 1)) {
                if right <= height {
                    continue;
                }
            }

            minima.push((i, j));
        }
    }

    minima
}

fn flood((i, j): (usize, usize), size: &mut i64, grid: &mut Vec<Vec<i64>>) {
    match grid.get_mut(i).and_then(|row| row.get_mut(j)) {
        None | Some(9) => return,
        Some(height) => {
            *height = 9;
            *size += 1;
        }
    }

    flood((i.wrapping_sub(1), j), size, grid);
    flood((i + 1, j), size, grid);
    flood((i, j.wrapping_sub(1)), size, grid);
    flood((i, j + 1), size, grid);
}
