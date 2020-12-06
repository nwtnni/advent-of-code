use aoc::*;

pub struct CustomCustoms(Vec<Vec<AsciiSet>>);

impl Fro for CustomCustoms {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split("\n\n")
            .map(|group| {
                group
                    .split_whitespace()
                    .map(AsciiSet::from)
                    .collect()
            })
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
                    .into_iter()
                    .fold(AsciiSet::none(), AsciiSet::or)
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
                    .into_iter()
                    .fold(AsciiSet::all(), AsciiSet::and)
                    .len()
            })
            .sum::<usize>()
            as i64
    }
}
