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
        Day::D01 => run!(day_01::CalorieCounting),
        Day::D02 => run!(day_02::RockPaperScissors),
        Day::D03 => run!(day_03::RucksackReorganization),
        Day::D04 => run!(day_04::CampCleanup),
        Day::D05 => run!(day_05::SupplyStacks),
        Day::D06 => run!(day_06::TuningTrouble),
        Day::D07 => run!(day_07::NoSpaceLeftOnDevice),
        Day::D08 => run!(day_08::TreetopTreeHouse),
        Day::D09 => run!(day_09::RopeBridge),
        Day::D10 => run!(day_10::CathodeRayTube),
        Day::D11 => run!(day_11::MonkeyInTheMiddle),
        Day::D12 => run!(day_12::HillClimbingAlgorithm),
        Day::D13 => run!(day_13::DistressSignal),
        Day::D14 => run!(day_14::RegolithReservoir),
        Day::D15 => run!(day_15::BeaconExclusionZone),
        Day::D16 => run!(day_16::ProboscideaVolcanium),
        Day::D17 => run!(day_17::PyroclasticFlow),
        Day::D18 => run!(day_18::BoilingBoulders),
        Day::D19 => run!(day_19::NotEnoughMinerals),
        Day::D20 => run!(day_20::GrovePositioningSystem),
        Day::D21 => run!(day_21::MonkeyMath),
        Day::D22 => run!(day_22::MonkeyMap),
        Day::D23 => run!(day_23::UnstableDiffusion),
        Day::D24 => run!(day_24::BlizzardBasin),
        Day::D25 => run!(day_25::FullOfHotAir),
    }
}
