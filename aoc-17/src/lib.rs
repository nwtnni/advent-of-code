use std::str;

mod day_01;
mod day_02;
mod day_03;

pub fn main(day: aoc::Day, part: aoc::Part, input: &str) -> Result<usize, aoc::Error> {
    use aoc::Parse;
    match day {
    | aoc::Day::D01 => day_01::InverseCaptcha::parse(&input),
    | aoc::Day::D02 => day_02::CorruptionChecksum::parse(&input),
    | aoc::Day::D03 => day_03::SpiralMemory::parse(&input),
    | _ => unimplemented!(),
    }.map(|mut sol| sol.solve(part))
}
