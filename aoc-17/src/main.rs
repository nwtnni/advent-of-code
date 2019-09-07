use std::fs;
use std::path;

use structopt::StructOpt;

mod day_01;
mod day_02;

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
    let opt = Opt::from_args();
    let input = fs::read_to_string(opt.file)?;
    use aoc::Day::*;
    let output = match opt.day {
    | D01 => run::<day_01::InverseCaptcha>(&input, opt.part)?,
    | D02 => run::<day_02::CorruptionChecksum>(&input, opt.part)?,
    | _ => unimplemented!(),
    };
    println!("{}", output);
    Ok(())
}

fn run<S: aoc::Solution>(input: &str, part: aoc::Part) -> Result<usize, failure::Error> {
    input.parse::<S>()
        .map(move |solution| solution.solve(part))
        .map_err(From::from)
}
