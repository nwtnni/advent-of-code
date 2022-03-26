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
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

pub fn solve(day: Day, part: Part, input: &str) -> i64 {
    macro_rules! run {
        ($solution:ty) => {
            <$solution>::run(input, part)
        };
    }
    match day {
        Day::D01 => run!(day_01::ReportRepair),
        Day::D02 => run!(day_02::PasswordPhilosophy),
        Day::D03 => run!(day_03::TobogganTrajectory),
        Day::D04 => run!(day_04::PassportProcessing),
        Day::D05 => run!(day_05::BinaryBoarding),
        Day::D06 => run!(day_06::CustomCustoms),
        Day::D07 => run!(day_07::HandyHaversacks),
        Day::D08 => run!(day_08::HandheldHalting),
        Day::D09 => run!(day_09::EncodingError),
        Day::D10 => run!(day_10::AdapterArray),
        Day::D11 => run!(day_11::SeatingSystem),
        Day::D12 => run!(day_12::RainRisk),
        Day::D13 => run!(day_13::ShuttleSearch),
        Day::D14 => run!(day_14::DockingData),
        Day::D15 => run!(day_15::RambunctiousRecitation),
        Day::D16 => run!(day_16::TicketTranslation),
        Day::D17 => run!(day_17::ConwayCubes),
        Day::D18 => run!(day_18::OperationOrder),
        Day::D19 => run!(day_19::MonsterMessages),
        Day::D20 => run!(day_20::JurassicJigsaw),
        Day::D21 => run!(day_21::AllergenAssessment),
        Day::D22 => run!(day_22::CrabCombat),
        Day::D23 => run!(day_23::CrabCups),
        Day::D24 => run!(day_24::LobbyLayout),
        Day::D25 => run!(day_25::ComboBreaker),
    }
}
