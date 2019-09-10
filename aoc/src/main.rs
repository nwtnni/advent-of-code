use std::path;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc-17", about = "Solutions to Advent of Code 2017")]
struct Opt {
    /// Which day's puzzle to solve.
    #[structopt(short = "d", long = "day")]
    day: aoc_core::Day,

    /// Whether to solve the first or second part of the puzzle.
    #[structopt(short = "p", long = "part")]
    part: aoc_core::Part,

    /// Path to puzzle's corresponding input.
    file: path::PathBuf,
}

pub fn main() {

}
