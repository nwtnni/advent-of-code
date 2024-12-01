use aoc::*;

#[derive(Clone, Debug)]
pub struct HistorianHysteria(Vec<(i64, i64)>);

impl Fro for HistorianHysteria {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let mut iter = line.split_whitespace();
                let l = iter.give().to();
                let r = iter.give().to();
                (l, r)
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for HistorianHysteria {
    fn one(self) -> i64 {
        let (mut ls, mut rs) = self.0.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();
        ls.sort();
        rs.sort();
        ls.into_iter()
            .zip(rs)
            .map(|(l, r)| l.abs_diff(r))
            .sum::<u64>() as i64
    }

    fn two(self) -> i64 {
        let (ls, rs) = self.0.into_iter().unzip::<_, _, Vec<_>, Counter<_>>();
        ls.into_iter()
            .map(|l| l * rs.get(&l).copied().unwrap_or(0) as i64)
            .sum()
    }
}
