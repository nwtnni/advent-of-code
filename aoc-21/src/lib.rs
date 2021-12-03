use aoc::*;

mod day_01;
mod day_02;
mod day_03;

pub fn solve(day: Day, part: Part, input: &str) -> i64 {
    macro_rules! run {
        ($solution:ty) => {
            <$solution>::run(input, part)
        }
    }
    match day {
    | Day::D01 => run!(day_01::SonarSweep),
    | Day::D02 => run!(day_02::Dive),
    | Day::D03 => run!(day_03::BinaryDiagnostic),
    | _ => unimplemented!(),
    }
}
