use std::fs;
use std::path;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc-17", about = "Solutions to Advent of Code 2017")]
struct Opt {
    /// Which year's calendar to solve.
    #[structopt(short = "y", long = "year")]
    year: aoc_core::Year,

    /// Which day's puzzle to solve.
    #[structopt(short = "d", long = "day")]
    day: aoc_core::Day,

    /// Whether to solve the first or second part of the puzzle.
    #[structopt(short = "p", long = "part")]
    part: aoc_core::Part,

    /// Path to puzzle's corresponding input.
    file: path::PathBuf,
}

pub fn main() -> Result<(), aoc_core::Error> {
    let opt = Opt::from_args();
    let txt = fs::read_to_string(opt.file).map_err(aoc_core::Error::IO)?;
    let out = match opt.year {
    | aoc_core::Year::Y17 => aoc_17::solve(opt.day, opt.part, &txt)?,
    | _ => unimplemented!(),
    };
    println!("{}", out);
    Ok(())
}
