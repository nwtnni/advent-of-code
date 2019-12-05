use std::str;

use aoc::*;

pub struct Placeholder(intcode::Program);

impl Fro for Placeholder {
    fn fro(input: &str) -> Self {
        Placeholder(intcode::Program::fro(input))
    }
}

impl Solution for Placeholder {
    fn one(mut self) -> i32 {
        let mut out = 0;
        self.0.run_io(|| 1, |output| out = output);
        out
    }

    fn two(mut self) -> i32 {
        let mut out = 0;
        self.0.run_io(|| 5, |output| out = output);
        out
    }
}
