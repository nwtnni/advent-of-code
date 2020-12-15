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
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;

pub fn solve(day: Day, part: Part, input: &str) -> i64 {
    macro_rules! run {
        ($solution:ty) => {
            <$solution>::run(input, part)
        }
    }
    match day {
    | Day::D01 => run!(day_01::ReportRepair),
    | Day::D02 => run!(day_02::PasswordPhilosophy),
    | Day::D03 => run!(day_03::TobogganTrajectory),
    | Day::D04 => run!(day_04::PassportProcessing),
    | Day::D05 => run!(day_05::BinaryBoarding),
    | Day::D06 => run!(day_06::CustomCustoms),
    | Day::D07 => run!(day_07::HandyHaversacks),
    | Day::D08 => run!(day_08::HandheldHalting),
    | Day::D09 => run!(day_09::EncodingError),
    | Day::D10 => run!(day_10::AdapterArray),
    | Day::D11 => run!(day_11::SeatingSystem),
    | Day::D12 => run!(day_12::RainRisk),
    | Day::D13 => run!(day_13::ShuttleSearch),
    | Day::D14 => run!(day_14::DockingData),
    | Day::D15 => run!(day_15::RambunctiousRecitation),
    | _ => unimplemented!(),
    }
}
