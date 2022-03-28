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

pub fn solve(day: aoc::Day, part: aoc::Part, input: &str) -> i64 {
    macro_rules! run {
        ($solution:ty) => {
            <$solution>::run(input, part)
        };
    }
    match day {
        Day::D01 => run!(day_01::NotQuiteLisp),
        Day::D02 => run!(day_02::IWasToldThereWouldBeNoMath),
        Day::D03 => run!(day_03::PerfectlySphericalHousesInAVacuum),
        Day::D04 => run!(day_04::TheIdealStockingStuffer),
        Day::D05 => run!(day_05::DoesntHeHaveInternElvesForThis),
        Day::D06 => run!(day_06::ProbablyAFireHazard),
        Day::D07 => run!(day_07::SomeAssemblyRequired),
        Day::D08 => run!(day_08::Matchsticks),
        Day::D09 => run!(day_09::AllInASingleNight),
        Day::D10 => run!(day_10::ElvesLookElvesSay),
        Day::D11 => run!(day_11::CorporatePolicy),
        Day::D12 => run!(day_12::JSAbacusFrameworkio),
        Day::D13 => run!(day_13::KnightsOfTheDinnerTable),
        Day::D14 => run!(day_14::ReindeerOlympics),
        Day::D15 => run!(day_15::ScienceForHungryPeople),
        Day::D16 => run!(day_16::AuntSue),
        Day::D17 => run!(day_17::NoSuchThingAsTooMuch),
        Day::D18 => run!(day_18::LikeAGIFForYourYard),
        Day::D19 => run!(day_19::MedicineForRudolph),
        Day::D20 => run!(day_20::InfiniteElvesAndInfiniteHouses),
        Day::D21 => run!(day_21::RPGSimulator20XX),
        Day::D22 => run!(day_22::WizardSimulator20XX),
        Day::D23 => run!(day_23::OpeningTheTuringLock),
        Day::D24 => run!(day_24::ItHangsInTheBalance),
        _ => unimplemented!(),
    }
}
