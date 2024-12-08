use aoc::*;

#[derive(Clone, Debug)]
pub struct BridgeRepair(Vec<Eq>);

#[derive(Clone, Debug)]
struct Eq {
    v: i64,
    ns: Vec<i64>,
}

impl Fro for BridgeRepair {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (v, ns) = line.split_once(": ").unwrap();
                let v = v.to::<i64>();
                let ns = ns.split_whitespace().map(i64::fro).collect();
                Eq { v, ns }
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for BridgeRepair {
    fn one(self) -> i64 {
        self.0
            .iter()
            .filter(|eq| solve(eq.v, &eq.ns))
            .map(|eq| eq.v)
            .sum()
    }

    fn two(self) -> i64 {
        self.0
            .iter()
            .filter(|eq| solve_concat(eq.v, &eq.ns))
            .map(|eq| eq.v)
            .sum()
    }
}

fn solve(v: i64, ns: &[i64]) -> bool {
    match ns {
        [] => unreachable!(),
        [n] => v == *n,
        [.., n] if v < *n => false,
        [ns @ .., n] => v % n == 0 && solve(v / n, ns) || solve(v - n, ns),
    }
}

fn solve_concat(v: i64, ns: &[i64]) -> bool {
    match ns {
        [] => unreachable!(),
        [n] => v == *n,
        [.., n] if v < *n => false,
        [ns @ .., n] => {
            // Split out bottom part of `v`
            let pow = 10i64.pow(n.ilog10() + 1);

            v % n == 0 && solve_concat(v / n, ns)
                || (v % pow == *n) && solve_concat(v / pow, ns)
                || solve_concat(v - n, ns)
        }
    }
}
