use aoc::*;

#[derive(Clone, Debug)]
pub struct SonarSweep(Vec<i64>);

impl Fro for SonarSweep {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(i64::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for SonarSweep {
    fn one(self) -> i64 {
        self.0
            .windows(2)
            .filter(|depths| depths[1] > depths[0])
            .count() as i64
    }

    fn two(self) -> i64 {
        let windows = self
            .0
            .windows(3)
            .map(|window| window.iter().sum::<i64>());

        windows
            .clone()
            .zip(windows.skip(1))
            .filter(|(prev, next)| next > prev)
            .count() as i64
    }
}
