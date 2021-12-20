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

pub fn solve(day: Day, part: Part, input: &str) -> i64 {
    macro_rules! run {
        ($solution:ty) => {
            <$solution>::run(input, part)
        }
    }
    match day {
    | Day::D01 => run!(day_01::SonarSweep),
    | Day::D02 => run!(day_02::Dive),
    | Day::D03 => run!(day_03::BinaryDiagnostic),
    | Day::D04 => run!(day_04::GiantSquid),
    | Day::D05 => run!(day_05::HydrothermalVenture),
    | Day::D06 => run!(day_06::Lanternfish),
    | Day::D07 => run!(day_07::TheTreacheryOfWhales),
    | Day::D08 => run!(day_08::SevenSegmentSearch),
    | Day::D09 => run!(day_09::SmokeBasin),
    | Day::D10 => run!(day_10::SyntaxScoring),
    | Day::D11 => run!(day_11::DumboOctopus),
    | Day::D12 => run!(day_12::PassagePathing),
    | Day::D13 => run!(day_13::TransparentOrigami),
    | Day::D14 => run!(day_14::ExtendedPolymerization),
    | Day::D15 => run!(day_15::Chiton),
    | Day::D16 => run!(day_16::PacketDecoder),
    | Day::D17 => run!(day_17::TrickShot),
    | Day::D18 => run!(day_18::Snailfish),
    | Day::D19 => run!(day_19::BeaconScanner),
    | Day::D20 => run!(day_20::TrenchMap),
    | _ => unimplemented!(),
    }
}
