use aoc::*;

#[derive(Clone, Debug)]
pub struct Placeholder(intcode::Program);

impl Fro for Placeholder {
    fn fro(input: &str) -> Self {
        Placeholder(intcode::Program::fro(input))
    }
}

impl Solution for Placeholder {
    fn one(mut self) -> i64 {
        self.0.pipe(1).unwrap()
    }

    fn two(mut self) -> i64 {
        self.0.pipe(2).unwrap()
    }
}
