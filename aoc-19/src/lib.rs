use aoc::*;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

pub fn solve(day: Day, part: Part, input: &str) -> i64 {
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
    | Day::D06 => run!(day_06::UniversalOrbitMap),
    | Day::D07 => run!(day_07::AmplificationCircuit),
    | Day::D08 => run!(day_08::SpaceImageFormat),
    | Day::D09 => run!(day_09::Placeholder),
    | _ => unimplemented!(),
    }
}
