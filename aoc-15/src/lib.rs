use std::str;

mod day_01;
mod day_02;

pub fn solve(day: aoc::Day, part: aoc::Part, input: &str) -> Result<i32, aoc::Error> {
    use aoc::Parse;
    match day {
    | aoc::Day::D01 => day_01::NotQuiteLisp::parse(input),
    | aoc::Day::D02 => day_02::IWasToldThereWouldBeNoMath::parse(input),
    | _ => unimplemented!(),
    }.map(|mut sol| sol.solve(part))
}
