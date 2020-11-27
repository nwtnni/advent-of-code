use std::fs;
use std::path;

use anyhow::anyhow;
use anyhow::Context as _;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "aoc", about = "Advent of Code CLI utility")]
struct Opt {
    /// Advent of Code session token
    ///
    /// https://github.com/wimglenn/advent-of-code-wim/issues/1
    #[structopt(long = "session-token", env = "SESSION_TOKEN")]
    token: Option<String>,

    #[structopt(short = "y", long = "year")]
    year: aoc_core::Year,

    #[structopt(short = "d", long = "day")]
    day: aoc_core::Day,

    #[structopt(short = "p", long = "part")]
    part: Option<aoc_core::Part>,

    #[structopt(subcommand)]
    command: Command,
}

#[derive(Clone, Debug, StructOpt)]
enum Command {
    /// Fetch puzzle description
    Description,

    /// Fetch puzzle input
    Input,

    /// Solve puzzle and print solution
    Solve {
        input: Option<path::PathBuf>,
    },

    /// Solve puzzle and submit solution
    Submit {
        input: Option<path::PathBuf>,
    },
}

pub fn main() -> anyhow::Result<()> {
    match Opt::from_args() {
    | Opt { token: _, year, day, part: Some(part), command: Command::Solve { input: Some(path) } } => {
        let txt = fs::read_to_string(&path)
            .with_context(|| anyhow!("Could not read file: '{}'", path.display()))?;

        let out = match year {
        | aoc_core::Year::Y15 => aoc_15::solve(day, part, &txt),
        | aoc_core::Year::Y17 => aoc_17::solve(day, part, &txt),
        | aoc_core::Year::Y19 => aoc_19::solve(day, part, &txt),
        | _ => unimplemented!(),
        };

        println!("{}", out);
    }
    | _ => unimplemented!(),
    }

    Ok(())
}
