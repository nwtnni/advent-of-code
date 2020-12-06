use aoc::*;

pub struct CustomCustoms(Vec<String>);

impl Fro for CustomCustoms {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split("\n\n")
            .map(String::from)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for CustomCustoms {
    fn one(self) -> i64 {
        self.0
            .into_iter()
            .map(|group| {
                group
                    .split_whitespace()
                    .map(AlphaSet::from)
                    .fold(AlphaSet::new(), AlphaSet::or)
                    .len()
            })
            .sum::<usize>()
            as i64
    }

    fn two(self) -> i64 {
        self.0
            .into_iter()
            .map(|group| {
                group
                    .split_whitespace()
                    .map(AlphaSet::from)
                    .fold(AlphaSet::all(), AlphaSet::and)
                    .len()
            })
            .sum::<usize>()
            as i64
    }
}
