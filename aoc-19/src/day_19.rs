use std::collections::HashSet;
use std::collections::HashMap;
use std::str;

use aoc::*;

#[derive(Clone, Debug)]
pub struct TractorBeam(intcode::Program);

impl Fro for TractorBeam {
    fn fro(input: &str) -> Self {
        TractorBeam(intcode::Program::fro(input))
    }
}

impl Solution for TractorBeam {
    fn one(mut self) -> i64 {
        let mut sum = 0;
        for y in 0..50 {
            for x in 0..50 {
                self.0.input(x).unwrap();
                self.0.input(y).unwrap();
                sum += self.0.output().unwrap();
                self.0.reset();
            }
        }
        sum
    }

    fn two(self) -> i64 {
        todo!()
    }
}
