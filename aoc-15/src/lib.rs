use std::str;

mod day_01;
mod day_02;
mod day_03;
mod day_04;

pub fn solve(day: aoc::Day, part: aoc::Part, input: &str) -> Result<i32, aoc::Error> {
    use aoc::Parse;
    match day {
    | aoc::Day::D01 => day_01::NotQuiteLisp::parse(input),
    | aoc::Day::D02 => day_02::IWasToldThereWouldBeNoMath::parse(input),
    | aoc::Day::D03 => day_03::PerfectlySphericalHousesInAVacuum::parse(input),
    | aoc::Day::D04 => day_04::TheIdealStockingStuffer::parse(input),
    | _ => unimplemented!(),
    }.map(|mut sol| sol.solve(part))
}
