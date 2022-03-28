use aoc::*;

#[derive(Clone, Debug)]
pub struct InfiniteElvesAndInfiniteHouses(i64);

impl Fro for InfiniteElvesAndInfiniteHouses {
    fn fro(input: &str) -> Self {
        input.trim().tap(i64::fro).tap(Self)
    }
}

impl Solution for InfiniteElvesAndInfiniteHouses {
    fn one(self) -> i64 {
        const BOUND: usize = 1_000_000;

        let mut sums = vec![10i64; BOUND];

        for factor in 2..BOUND {
            for block in sums.chunks_mut(factor as usize).skip(1) {
                block[0] += factor as i64 * 10;
            }
        }

        sums.iter().position(|sum| *sum >= self.0).unwrap() as i64
    }

    fn two(self) -> i64 {
        const BOUND: usize = 1_000_000;

        let mut sums = vec![10i64; BOUND];

        for factor in 2..BOUND {
            for block in sums.chunks_mut(factor as usize).skip(1).take(50) {
                block[0] += factor as i64 * 11;
            }
        }

        sums.iter().position(|sum| *sum >= self.0).unwrap() as i64
    }
}
