mod day_01;
mod day_02;
mod day_03;
mod day_04;

pub fn solve(day: aoc::Day, part: aoc::Part, input: &str) -> Result<i32, aoc::Error> {
    macro_rules! run {
        ($solution:ty) => {
            aoc::run::<$solution>(input, part)
        }
    }
    match day {
    | aoc::Day::D01 => run!(day_01::TheTyrannyOfTheRocketEquation),
    | aoc::Day::D02 => run!(day_02::ProgramAlarm),
    | aoc::Day::D03 => run!(day_03::CrossedWires),
    | aoc::Day::D04 => run!(day_04::SecureContainer),
    | _ => unimplemented!(),
    }
}
