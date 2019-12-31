use std::str;

use aoc::*;

pub struct SlamShuffle(Vec<Shuffle>);

/// Linear function of position: ax + b
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Shuffle {
    a: i64,
    b: i64,
}

impl Default for Shuffle {
    /// Identity shuffle.
    fn default() -> Self {
        Shuffle {
            a: 1,
            b: 0,
        }
    }
}

impl Shuffle {
    pub fn flip() -> Self {
        Shuffle { a: -1, b: -1 }
    }

    pub fn cut(n: i64) -> Self {
        Shuffle { a: 1, b: -n }
    }

    pub fn deal(n: i64) -> Self {
        Shuffle { a: n, b: 0 }
    }

    /// f(x) = ax + b
    /// g(x) = cx + d
    ///
    /// (g ⋅ f) = c(ax + b) + d
    ///         = acx + bc + d
    ///         = (ac)x + (bc + d)
    pub fn compose(self, with: Self, modulo: i64) -> Self {
        use std::ops::Mul;
        use std::ops::Add;
        Shuffle {
            a: self.a
                .mul(with.a)
                .rem_euclid(modulo),
            b: self.b
                .mul(with.a)
                .rem_euclid(modulo)
                .add(with.b)
                .rem_euclid(modulo),
        }
    }

    pub fn apply(self, pos: i64, modulo: i64) -> i64 {
        use std::ops::Add;
        use std::ops::Mul;
        self.a
            .mul(pos)
            .rem_euclid(modulo)
            .add(self.b)
            .rem_euclid(modulo)
    }
}

impl Fro for SlamShuffle {
    fn fro(input: &str) -> Self {
        let mut shuffle = Vec::new();
        for line in input.trim().split('\n') {
            let mut iter = line.trim().split_whitespace();
            match iter.give() {
            | "deal" => {
                match iter.give() {
                | "with" => shuffle.push(Shuffle::deal(iter.nth(1).unwrap().to())),
                | "into" => shuffle.push(Shuffle::flip()),
                | _ => unreachable!(),
                }
            }
            | "cut" => shuffle.push(Shuffle::cut(iter.give().to())),
            | _ => unreachable!(),
            }
        }
        SlamShuffle(shuffle)
    }
}

impl Solution for SlamShuffle {
    fn one(self) -> i64 {
        const M: i64 = 10007;
        self.0
            .into_iter()
            .fold(Shuffle::default(), |acc, shuffle| acc.compose(shuffle, M))
            .apply(2019, M)
    }

    fn two(self) -> i64 {
        const _M: i64 = 119315717514047;
        const _S: i64 = 101741582076661;
        todo!()
    }
}
