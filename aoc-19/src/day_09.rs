use aoc::*;

#[derive(Clone, Debug)]
pub struct SensorBoost(intcode::Program);

impl Fro for SensorBoost {
    fn fro(input: &str) -> Self {
        SensorBoost(intcode::Program::fro(input))
    }
}

impl Solution for SensorBoost {
    fn one(mut self) -> i64 {
        self.0.pipe(1).unwrap()
    }

    fn two(mut self) -> i64 {
        self.0.pipe(2).unwrap()
    }
}
