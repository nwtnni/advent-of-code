use std::iter;
use std::str;

use aoc::*;

pub struct Placeholder(intcode::Program);

impl Fro for Placeholder {
    fn fro(input: &str) -> Self {
        Placeholder(intcode::Program::fro(input))
    }
}

impl Solution for Placeholder {
    fn one(self) -> i32 {
        self.0.run_with_input(iter::repeat(1))
    }

    fn two(self) -> i32 {
        self.0.run_with_input(iter::repeat(5))
    }
}
