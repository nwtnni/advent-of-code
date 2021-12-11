use aoc::*;

#[derive(Clone)]
pub struct DumboOctopus(Vec<Vec<i64>>);

impl Fro for DumboOctopus {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| line.chars().map(|char| i64::from(char as u8 - b'0')).collect())
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for DumboOctopus {
    fn one(mut self) -> i64 {
        let h = self.0.len();
        let w = self.0[0].len();

        let mut flashes = 0;
        let mut flashed = Vec::new();

        for _ in 0..100 {
            for i in 0..h {
                for j in 0..w {
                    self.0[i][j] += 1;
                    if self.0[i][j] > 9 {
                        self.0[i][j] = 0;
                        flashed.push((i, j));
                        flashes += 1;
                    }
                }
            }

            while let Some((i, j)) = flashed.pop() {
                for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }

                        let i = i as i64 + di;
                        let j = j as i64 + dj;

                        if i < 0 || i as usize > h - 1
                        || j < 0 || j as usize > w - 1 {
                            continue;
                        }

                        let i = i as usize;
                        let j = j as usize;

                        match self.0[i][j] {
                            0 => (),
                            9 => {
                                self.0[i][j] = 0;
                                flashes += 1;
                                flashed.push((i, j));
                            }
                            _ => self.0[i][j] += 1,
                        }
                    }
                }
            }
        }

        flashes
    }

    fn two(mut self) -> i64 {
        let h = self.0.len();
        let w = self.0[0].len();

        let mut flashed = Vec::new();

        for step in 1.. {

            let mut flashes = 0;

            for i in 0..h {
                for j in 0..w {
                    self.0[i][j] += 1;
                    if self.0[i][j] > 9 {
                        self.0[i][j] = 0;
                        flashed.push((i, j));
                        flashes += 1;
                    }
                }
            }

            while let Some((i, j)) = flashed.pop() {
                for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }

                        let i = i as i64 + di;
                        let j = j as i64 + dj;

                        if i < 0 || i as usize > h - 1
                        || j < 0 || j as usize > w - 1 {
                            continue;
                        }

                        let i = i as usize;
                        let j = j as usize;

                        match self.0[i][j] {
                            0 => (),
                            9 => {
                                self.0[i][j] = 0;
                                flashes += 1;
                                flashed.push((i, j));
                            }
                            _ => self.0[i][j] += 1,
                        }
                    }
                }
            }

            if flashes == w * h {
                return step;
            }
        }

        unreachable!()
    }
}
