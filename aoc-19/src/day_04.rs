use std::collections::HashSet;
use std::str;

use aoc::*;

#[derive(Clone, Debug)]
pub struct SecureContainer {
    lo: i32,
    hi: i32,
}

impl Fro for SecureContainer {
    fn fro(input: &str) -> Self {
        let mut iter = input.trim().split('-');
        let lo = iter
            .give()
            .to::<i32>();
        let hi = iter
            .give()
            .to::<i32>();
        SecureContainer {
            lo,
            hi,
        }
    }
}

impl Solution for SecureContainer {
    fn one(self) -> i32 {

        let mut count = 0;

        'outer: for i in self.lo..self.hi {
            let mut prev = None;
            let mut double = false;
            let mut number = i;

            for place in &[100000, 10000, 1000, 100, 10, 1] {
                let next = number / place;
                number %= place;

                if next < prev.unwrap_or(-1) { continue 'outer }

                double |= prev == Some(next);
                prev = Some(next);
            }

            if double {
                count += 1;
            }
        }

        count
    }

    fn two(self) -> i32 {

        let mut count = 0;
        let mut double = HashSet::new();
        let mut triple = HashSet::new();

        'outer: for i in self.lo..self.hi {
            double.clear();
            triple.clear();

            let mut prev = [None; 2];
            let mut number = i;

            for place in &[100000, 10000, 1000, 100, 10, 1] {
                let next = number / place;
                number %= place;

                if next < prev[0].unwrap_or(-1) { continue 'outer; }

                if Some(next) == prev[0] {
                    double.insert(next);
                    if Some(next) == prev[1] {
                        triple.insert(next);
                    }
                }

                prev[1] = prev[0];
                prev[0] = Some(next);
            }

            if double.difference(&triple).count() > 0 {
                count += 1;
            }
        }
        count
    }
}
