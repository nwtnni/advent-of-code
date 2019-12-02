use std::str;

pub struct ProgramAlarm(aoc::intcode::Program);

impl str::FromStr for ProgramAlarm {
    type Err = aoc::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        aoc::intcode::Program::from_str(input)
            .map(ProgramAlarm)
    }
}

impl aoc::Solution for ProgramAlarm {
    fn one(mut self) -> i32 {
        self.0.run(12, 2)
    }

    fn two(mut self) -> i32 {
        for noun in 0..100 {
            for verb in 0..100 {
                let mut program = self.0.clone();
                if program.run(noun, verb) == 19690720 {
                    return noun * 100 + verb;
                }
            }
        }
        unreachable!()
    }
}
