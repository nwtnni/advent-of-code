use std::fs;
use std::path;
use std::str;

use structopt::StructOpt;

mod day_01;
mod day_02;
mod day_03;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc-17", about = "Solutions to Advent of Code 2017")]
struct Opt {
    /// Which day's puzzle to solve.
    #[structopt(short = "d", long = "day")]
    day: aoc::Day,

    /// Whether to solve the first or second part of the puzzle.
    #[structopt(short = "p", long = "part")]
    part: aoc::Part,

    /// Path to puzzle's corresponding input.
    file: path::PathBuf,
}

pub fn main() -> Result<(), failure::Error> {
    use aoc::Day::*;
    let opt = Opt::from_args();
    let input = fs::read_to_string(opt.file)?;
    let mut sol = match opt.day {
    | D01 => parse::<day_01::InverseCaptcha>(&input)?,
    | D02 => parse::<day_02::CorruptionChecksum>(&input)?,
    | D03 => parse::<day_03::SpiralMemory>(&input)?,
    | _ => unimplemented!(),
    };
    let out = sol.solve(opt.part);
    println!("{}", out);
    Ok(())
}

fn parse<S>(input: &str) -> Result<Box<dyn aoc::Solution>, aoc::Error>
    where S: 'static +
        aoc::Solution +
        str::FromStr<Err = aoc::Error>,
{
  S::from_str(input)
    .map(Box::new)
    .map(|sol| sol as Box<dyn aoc::Solution>)
    .map(Result::Ok)?
}
