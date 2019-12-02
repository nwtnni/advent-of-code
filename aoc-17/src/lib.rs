use std::str;

mod day_01;
mod day_02;
mod day_03;

pub fn solve(day: aoc::Day, part: aoc::Part, input: &str) -> Result<i32, aoc::Error> {
    macro_rules! run {
        ($solution:ty) => {
            aoc::run::<$solution>(input, part)
        }
    }
    match day {
    | aoc::Day::D01 => run!(day_01::InverseCaptcha),
    | aoc::Day::D02 => run!(day_02::CorruptionChecksum),
    | aoc::Day::D03 => run!(day_03::SpiralMemory),
    | _ => unimplemented!(),
    }
}
