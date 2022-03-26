use aoc::*;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

pub fn solve(day: aoc::Day, part: aoc::Part, input: &str) -> i64 {
    macro_rules! run {
        ($solution:ty) => {
            <$solution>::run(input, part)
        };
    }
    match day {
        Day::D01 => run!(day_01::ChronalCalibration),
        Day::D02 => run!(day_02::InventoryManagementSystem),
        Day::D03 => run!(day_03::NoMatterHowYouSliceIt),
        Day::D04 => run!(day_04::ReposeRecord),
        Day::D05 => run!(day_05::AlchemicalReduction),
        _ => unimplemented!(),
    }
}
