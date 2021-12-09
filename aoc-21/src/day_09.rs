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
        let h = self.0.len();
        let w = self.0[0].len();

        let mut risk = 0;

        for i in 0..h {
            for j in 0..w {
                let height = self.0[i][j];

                if let Some(&up) = self.0.get(i.wrapping_sub(1)).and_then(|row| row.get(j)) {
                    if up <= height {
                        continue;
                    }
                }

                if let Some(&down) = self.0.get(i + 1).and_then(|row| row.get(j)) {
                    if down <= height {
                        continue;
                    }
                }

                if let Some(&left) = self.0.get(i).and_then(|row| row.get(j.wrapping_sub(1))) {
                    if left <= height {
                        continue;
                    }
                }

                if let Some(&right) = self.0.get(i).and_then(|row| row.get(j + 1)) {
                    if right <= height {
                        continue;
                    }
                }

                risk += height + 1;
            }
        }

        risk
    }

    fn two(mut self) -> i64 {
        let h = self.0.len();
        let w = self.0[0].len();

        let mut minima = Vec::new();

        for i in 0..h {
            for j in 0..w {
                let height = self.0[i][j];

                if let Some(&up) = self.0.get(i.wrapping_sub(1)).and_then(|row| row.get(j)) {
                    if up <= height {
                        continue;
                    }
                }

                if let Some(&down) = self.0.get(i + 1).and_then(|row| row.get(j)) {
                    if down <= height {
                        continue;
                    }
                }

                if let Some(&left) = self.0.get(i).and_then(|row| row.get(j.wrapping_sub(1))) {
                    if left <= height {
                        continue;
                    }
                }

                if let Some(&right) = self.0.get(i).and_then(|row| row.get(j + 1)) {
                    if right <= height {
                        continue;
                    }
                }

                minima.push((i, j));
            }
        }

        fn flood((i, j): (usize, usize), count: &mut i64, grid: &mut Vec<Vec<i64>>) {
            if grid[i][j] >= 9 {
                return;
            }

            grid[i][j] = 9;
            *count += 1;

            if let Some(up) = grid.get_mut(i.wrapping_sub(1)).and_then(|row| row.get_mut(j)) {
                if *up <= 9 {
                    flood((i.wrapping_sub(1), j), count, grid);
                }
            }

            if let Some(down) = grid.get_mut(i + 1).and_then(|row| row.get_mut(j)) {
                if *down <= 9 {
                    flood((i + 1, j), count, grid);
                }
            }

            if let Some(left) = grid.get_mut(i).and_then(|row| row.get_mut(j.wrapping_sub(1))) {
                if *left <= 9 {
                    flood((i, j.wrapping_sub(1)), count, grid);
                }
            }

            if let Some(right) = grid.get_mut(i).and_then(|row| row.get_mut(j + 1)) {
                if *right <= 9 {
                    flood((i, j + 1), count, grid);
                }
            }
        }

        let mut basins = Vec::new();

        for point in minima {
            let mut count = 0;
            flood(point, &mut count, &mut self.0);
            basins.push(count);
        }

        basins.sort();
        basins.into_iter().rev().take(3).product::<i64>()
    }
}
