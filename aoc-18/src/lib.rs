use aoc::*;

mod day_01;
mod day_02;

pub fn solve(day: aoc::Day, part: aoc::Part, input: &str) -> i64 {
    macro_rules! run {
        ($solution:ty) => {
            <$solution>::run(input, part)
        };
    }
    match day {
    | Day::D01 => run!(day_01::ChronalCalibration),
    | Day::D02 => run!(day_02::InventoryManagementSystem),
        _ => unimplemented!(),
    }
}
