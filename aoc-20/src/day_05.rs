use std::cmp;

use aoc::*;

#[derive(Clone, Debug)]
pub struct BinaryBoarding(Vec<usize>);

impl Fro for BinaryBoarding {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split_whitespace()
            .map(|pass| {
                pass.chars()
                    .map(|bit| match bit {
                        'F' | 'L' => '0',
                        'B' | 'R' => '1',
                        _ => unreachable!(),
                    })
                    .collect::<String>()
                    .tap(|pass| usize::from_str_radix(&pass, 2))
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for BinaryBoarding {
    fn one(self) -> i64 {
        self.0.into_iter().fold(0, cmp::max) as i64
    }

    fn two(mut self) -> i64 {
        self.0.sort_unstable();

        for passes in self.0.windows(2) {
            if passes[0] + 2 == passes[1] {
                return passes[0] as i64 + 1;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {

    use aoc::Fro as _;
    use aoc::Solution as _;

    #[test]
    fn part_one() {
        assert_eq!(super::BinaryBoarding::fro("FBFBBFFRLR").one(), 357);
    }
}
