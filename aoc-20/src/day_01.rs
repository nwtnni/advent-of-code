use aoc::*;

pub struct ReportRepair(Vec<i64>);

impl Fro for ReportRepair {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split_whitespace()
            .map(i64::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for ReportRepair {
    fn one(self) -> i64 {
        for i in 0..self.0.len() {
            for j in i + 1..self.0.len() {
                if self.0[i] + self.0[j] == 2020 {
                    return (self.0[i] * self.0[j]) as i64;
                }
            }
        }
        unreachable!()
    }

    fn two(self) -> i64 {
        for i in 0..self.0.len() {
            for j in i + 1..self.0.len() {
                for k in j + 1..self.0.len() {
                    if self.0[i] + self.0[j] + self.0[k] == 2020 {
                        return (self.0[i] * self.0[j] * self.0[k]) as i64;
                    }
                }
            }
        }
        unreachable!()
    }
}
