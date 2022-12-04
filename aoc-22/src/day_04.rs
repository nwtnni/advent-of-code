use aoc::*;

#[derive(Clone, Debug)]
pub struct CampCleanup(Vec<(i64, i64, i64, i64)>);

impl Fro for CampCleanup {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (a, b) = line.split_once(',').unwrap();
                let (_a, _b) = a.split_once('-').unwrap();
                let (_c, _d) = b.split_once('-').unwrap();
                (
                    _a.parse().unwrap(),
                    _b.parse().unwrap(),
                    _c.parse().unwrap(),
                    _d.parse().unwrap(),
                )
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for CampCleanup {
    fn one(self) -> i64 {
        self.0
            .into_iter()
            .filter(|(a, b, c, d)| a <= c && b >= d || c <= a && d >= b)
            .count() as i64
    }

    fn two(self) -> i64 {
        self.0
            .into_iter()
            .filter(|(a, b, c, d)| a <= c && b >= c || c <= a && d >= a)
            .count() as i64
    }
}
