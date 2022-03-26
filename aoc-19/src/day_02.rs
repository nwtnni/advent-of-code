use std::str;

use aoc::intcode;
use aoc::*;

pub struct ProgramAlarm(intcode::Program);

impl Fro for ProgramAlarm {
    fn fro(input: &str) -> Self {
        input.to::<intcode::Program>().tap(ProgramAlarm)
    }
}

impl Solution for ProgramAlarm {
    fn one(mut self) -> i64 {
        self.0.run_nv(12, 2)
    }

    fn two(mut self) -> i64 {
        for noun in 0..100 {
            for verb in 0..100 {
                self.0.reset();
                if self.0.run_nv(noun, verb) == 19690720 {
                    return noun * 100 + verb;
                }
            }
        }
        unreachable!()
    }
}
