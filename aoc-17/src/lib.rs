use std::str;

use aoc::Solution;

mod day_01;
mod day_02;
mod day_03;

pub fn solve(day: aoc::Day, part: aoc::Part, input: &str) -> i32 {
    macro_rules! run {
        ($solution:ty) => {
            <$solution>::run(input, part)
        }
    }
    match day {
    | aoc::Day::D01 => run!(day_01::InverseCaptcha),
    | aoc::Day::D02 => run!(day_02::CorruptionChecksum),
    | aoc::Day::D03 => run!(day_03::SpiralMemory),
    | _ => unimplemented!(),
    }
}
