use aoc::*;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

pub fn solve(day: Day, part: Part, input: &str) -> i32 {
    macro_rules! run {
        ($solution:ty) => {
            <$solution>::run(input, part)
        }
    }
    match day {
    | Day::D01 => run!(day_01::TheTyrannyOfTheRocketEquation),
    | Day::D02 => run!(day_02::ProgramAlarm),
    | Day::D03 => run!(day_03::CrossedWires),
    | Day::D04 => run!(day_04::SecureContainer),
    | Day::D05 => run!(day_05::SunnyWithAChanceOfAsteroids),
    | Day::D06 => run!(day_06::Placeholder),
    | _ => unimplemented!(),
    }
}
