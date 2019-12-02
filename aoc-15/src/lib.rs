use std::str;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

pub fn solve(day: aoc::Day, part: aoc::Part, input: &str) -> Result<i32, aoc::Error> {
    macro_rules! run {
        ($solution:ty) => {
            aoc::run::<$solution>(input, part)
        }
    }
    match day {
    | aoc::Day::D01 => run!(day_01::NotQuiteLisp),
    | aoc::Day::D02 => run!(day_02::IWasToldThereWouldBeNoMath),
    | aoc::Day::D03 => run!(day_03::PerfectlySphericalHousesInAVacuum),
    | aoc::Day::D04 => run!(day_04::TheIdealStockingStuffer),
    | aoc::Day::D05 => run!(day_05::DoesntHeHaveInternElvesForThis),
    | _ => unimplemented!(),
    }
}
