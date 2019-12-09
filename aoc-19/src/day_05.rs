use std::str;

use aoc::*;

pub struct SunnyWithAChanceOfAsteroids(intcode::Program);

impl Fro for SunnyWithAChanceOfAsteroids {
    fn fro(input: &str) -> Self {
        SunnyWithAChanceOfAsteroids(intcode::Program::fro(input))
    }
}

impl Solution for SunnyWithAChanceOfAsteroids {
    fn one(mut self) -> i64 {
        let mut out = 0;
        self.0.run_io(|| 1, |output| out = output);
        out
    }

    fn two(mut self) -> i64 {
        let mut out = 0;
        self.0.run_io(|| 5, |output| out = output);
        out
    }
}
