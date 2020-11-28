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
    token: String,

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
    | Opt { part: Some(_), command: Command::Description, .. } => {
        return Err(anyhow!("[USAGE ERROR]: subcommand `description` does not use flag `--part`"));
    }
    | Opt { part: Some(_), command: Command::Input, .. } => {
        return Err(anyhow!("[USAGE ERROR]: subcommand `input` does not use flag `--part`"));
    }
    | Opt { part: None, command: Command::Solve { .. }, .. } => {
        return Err(anyhow!("[USAGE ERROR]: subcommand `solve` requires flag `--part`"));
    }
    | Opt { part: None, command: Command::Submit { .. }, .. } => {
        return Err(anyhow!("[USAGE ERROR]: subcommand `submit` requires flag `--part`"));
    }

    | Opt { token, year, day, part: None, command: Command::Description } => {
        let client = aoc::api::Client::new(token)?;
        let description = client.description(year, day)?;
        println!("{}", description);
    }
    | Opt { token, year, day, part: None, command: Command::Input } => {
        let client = aoc::api::Client::new(token)?;
        let input = client.input(year, day)?;
        println!("{}", input);
    }
    | Opt { token, year, day, part: Some(part), command: Command::Solve { input } } => {
        let input = match input {
        | Some(path) => read(&path)?,
        | None => aoc::api::Client::new(token)?.input(year, day)?,
        };

        let output = solve(year, day, part, &input);

        println!("{}", output);
    }
    | Opt { token, year, day, part: Some(part), command: Command::Submit { input } } => {
        let client = aoc::api::Client::new(token)?;

        let input = match input {
        | Some(path) => read(&path)?,
        | None => client.input(year, day)?,
        };

        let output = solve(year, day, part, &input);

        if client.submit(year, day, part, output)? {
            println!("{} was correct!", output);
        } else {
            println!("{} was incorrect!", output);
        }
    }
    }

    Ok(())
}

fn read(path: &path::Path) -> anyhow::Result<String> {
    fs::read_to_string(&path)
        .with_context(|| anyhow!("Could not read file: '{}'", path.display()))
}

fn solve(
    year: aoc_core::Year,
    day: aoc_core::Day,
    part: aoc_core::Part,
    input: &str,
) -> i64 {
    match year {
    | aoc_core::Year::Y15 => aoc_15::solve(day, part, &input),
    | aoc_core::Year::Y17 => aoc_17::solve(day, part, &input),
    | aoc_core::Year::Y19 => aoc_19::solve(day, part, &input),
    | _ => unimplemented!(),
    }
}
