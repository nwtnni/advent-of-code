use aoc::*;

#[derive(Clone, Debug)]
pub struct RedNosedReports(Vec<Vec<i64>>);

impl Fro for RedNosedReports {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| line.split_whitespace().map(i64::fro).collect())
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for RedNosedReports {
    fn one(self) -> i64 {
        self.0.iter().filter(|level| safe(level)).count() as i64
    }

    fn two(self) -> i64 {
        self.0
            .into_iter()
            .filter(|level| {
                (0..level.len()).any(|problem| {
                    let mut level = level.clone();
                    level.remove(problem);
                    safe(&level)
                })
            })
            .count() as i64
    }
}

fn safe(level: &[i64]) -> bool {
    let mut increase = true;
    let mut decrease = true;
    for window in level.windows(2) {
        let delta = window[1] - window[0];
        increase &= (1..=3).contains(&delta);
        decrease &= (-3..=-1).contains(&delta);
    }
    increase | decrease
}
