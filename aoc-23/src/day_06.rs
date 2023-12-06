use aoc::*;

#[derive(Clone, Debug)]
pub struct WaitForIt(Vec<Race>, (i64, i64));

#[derive(Clone, Debug)]
struct Race {
    t: i64,
    d: i64,
}

impl Race {
    fn win(&self) -> i64 {
        (1..self.t).filter(|w| *w * (self.t - *w) > self.d).count() as i64
    }

    fn win_fast(&self) -> i64 {
        let lower = (1..self.t).find(|w| *w * (self.t - *w) > self.d).unwrap();

        let upper = (1..self.t)
            .rev()
            .find(|w| *w * (self.t - *w) > self.d)
            .unwrap();

        upper - lower + 1
    }
}

impl Fro for WaitForIt {
    fn fro(input: &str) -> Self {
        let mut iter = input.trim().split('\n');
        let a = iter.give();
        let b = iter.give();

        let t = a
            .chars()
            .filter(|a| a.is_numeric())
            .collect::<String>()
            .to::<i64>();
        let d = b
            .chars()
            .filter(|a| a.is_numeric())
            .collect::<String>()
            .to::<i64>();

        let ts = a
            .trim_start_matches("Time:")
            .split_whitespace()
            .map(i64::fro)
            .collect::<Vec<_>>();
        let ds = b
            .trim_start_matches("Distance:")
            .split_whitespace()
            .map(i64::fro)
            .collect::<Vec<_>>();
        let rs = std::iter::zip(ts, ds)
            .map(|(t, d)| Race { t, d })
            .collect::<Vec<_>>();

        Self(rs, (t, d))
    }
}

impl Solution for WaitForIt {
    fn one(self) -> i64 {
        self.0.iter().map(Race::win).product()
    }

    fn two(self) -> i64 {
        let (t, d) = self.1;
        let r = Race { t, d };
        r.win_fast()
    }
}
