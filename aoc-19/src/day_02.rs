use std::str;

use aoc::*;
use aoc::intcode;

pub struct ProgramAlarm(intcode::Program);

impl Fro for ProgramAlarm {
    fn fro(input: &str) -> Self {
        input.to::<intcode::Program>().tap(ProgramAlarm)
    }
}

impl Solution for ProgramAlarm {
    fn one(self) -> i32 {
        self.0.run_nv(12, 2)
    }

    fn two(self) -> i32 {
        for noun in 0..100 {
            for verb in 0..100 {
                let program = self.0.clone();
                if program.run_nv(noun, verb) == 19690720 {
                    return noun * 100 + verb;
                }
            }
        }
        unreachable!()
    }
}
