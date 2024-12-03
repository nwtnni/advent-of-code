use aoc::*;

mod day_01;
mod day_02;
mod day_03;

pub fn solve(day: Day, part: Part, input: &str) -> i64 {
    macro_rules! run {
        ($solution:ty) => {
            <$solution>::run(input, part)
        };
    }
    match day {
        Day::D01 => run!(day_01::HistorianHysteria),
        Day::D02 => run!(day_02::RedNosedReports),
        Day::D03 => run!(day_03::MullItOver),
        _ => unreachable!(),
    }
}
