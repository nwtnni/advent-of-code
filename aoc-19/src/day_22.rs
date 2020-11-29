use std::str;

use aoc::*;

pub struct SlamShuffle(Vec<Shuffle>);

/// Linear function of position: ax + b
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Shuffle {
    a: i128,
    b: i128,
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
        Shuffle { a: 1, b: -n as i128 }
    }

    pub fn deal(n: i64) -> Self {
        Shuffle { a: n as i128, b: 0 }
    }

    /// f(x) = ax + b
    /// g(x) = cx + d
    ///
    /// (g ⋅ f) = c(ax + b) + d
    ///         = acx + bc + d
    ///         = (ac)x + (bc + d)
    pub fn compose(self, with: Self, modulo: i128) -> Self {
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

    /// f(x) = ax + b (mod m)
    pub fn apply(self, p: i128, m: i128) -> i128 {
        use std::ops::Add;
        use std::ops::Mul;
        self.a
            .mul(p)
            .rem_euclid(m)
            .add(self.b)
            .rem_euclid(m)
    }

    ///  f^-1(x) = (x - b) * a^-1 (mod m)
    pub fn invert(self, pos: i128, modulo: i128) -> i128 {
        use std::ops::Sub;
        use std::ops::Mul;
        pos.sub(self.b)
            .rem_euclid(modulo)
            .mul(mod_inv(self.a as i64, modulo as i64) as i128)
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
        const M: i128 = 10007;
        self.0
            .into_iter()
            .fold(Shuffle::default(), |acc, shuffle| acc.compose(shuffle, M))
            .apply(2019, M)
            as i64
    }

    fn two(self) -> i64 {
        const M: i128 = 119315717514047;
        const S: i128 = 101741582076661;

        let mut double = self.0
            .into_iter()
            .fold(Shuffle::default(), |acc, shuffle| acc.compose(shuffle, M));

        let mut total = Shuffle::default();

        let mut mask = 0b1;
        while mask <= S {
            if S & mask > 0 {
                total = total.compose(double, M);
            }
            double = double.compose(double, M);
            mask <<= 1;
        }

        total.invert(2020, M) as i64
    }
}
