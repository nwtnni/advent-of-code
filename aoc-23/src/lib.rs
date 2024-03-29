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

pub fn solve(day: Day, part: Part, input: &str) -> i64 {
    macro_rules! run {
        ($solution:ty) => {
            <$solution>::run(input, part)
        };
    }
    match day {
        Day::D01 => run!(day_01::Trebuchet),
        Day::D02 => run!(day_02::CubeConundrum),
        Day::D03 => run!(day_03::GearRatios),
        Day::D04 => run!(day_04::Scratchcards),
        Day::D05 => run!(day_05::IfYouGiveASeedAFertilizer),
        Day::D06 => run!(day_06::WaitForIt),
        Day::D07 => run!(day_07::CamelCards),
        Day::D08 => run!(day_08::HauntedWasteland),
        Day::D09 => run!(day_09::MirageMaintenance),
        Day::D10 => run!(day_10::PipeMaze),
        Day::D11 => run!(day_11::CosmicExpansion),
        Day::D12 => run!(day_12::HotSprings),
        Day::D13 => run!(day_13::PointOfIncidence),
        Day::D14 => run!(day_14::ParabolicReflectorDish),
        Day::D15 => run!(day_15::LensLibrary),
        Day::D16 => run!(day_16::TheFloorWillBeLava),
        Day::D17 => run!(day_17::ClumsyCrucible),
        Day::D18 => run!(day_18::LavaductLagoon),
        Day::D19 => run!(day_19::Aplenty),
        Day::D20 => run!(day_20::PulsePropagation),
        Day::D21 => run!(day_21::StepCounter),
        _ => unreachable!(),
    }
}
