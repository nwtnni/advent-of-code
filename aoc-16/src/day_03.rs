use aoc::*;

#[derive(Clone, Debug)]
pub struct SquaresWithThreeSides(Vec<[i64; 3]>);

impl Fro for SquaresWithThreeSides {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let mut iter = line.trim().split_whitespace();
                [
                    iter.give().to::<i64>(),
                    iter.give().to::<i64>(),
                    iter.give().to::<i64>(),
                ]
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for SquaresWithThreeSides {
    fn one(self) -> i64 {
        self.0.iter().copied().filter(possible).count() as i64
    }

    fn two(self) -> i64 {
        self.0
            .chunks_exact(3)
            .map(|chunk| {
                (0..3)
                    .map(|index| [chunk[0][index], chunk[1][index], chunk[2][index]])
                    .filter(possible)
                    .count()
            })
            .sum::<usize>() as i64
    }
}

fn possible(triangle: &[i64; 3]) -> bool {
    triangle[0] < triangle[1] + triangle[2]
        && triangle[1] < triangle[0] + triangle[2]
        && triangle[2] < triangle[0] + triangle[1]
}
