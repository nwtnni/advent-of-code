use std::fs;
use std::path;

use anyhow::anyhow;
use anyhow::Context as _;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "aoc", about = "Advent of Code CLI utility")]
struct Opt {
    /// Advent of Code account name
    ///
    /// This is an arbitrary string used to associate the input and submission
    /// caches for a single user across different session tokens.
    #[structopt(short, long, env = "AOC_ACCOUNT")]
    account: String,

    /// Advent of Code session token
    ///
    /// https://github.com/wimglenn/advent-of-code-wim/issues/1
    #[structopt(short, long = "session-token", env = "AOC_SESSION_TOKEN")]
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

    /// Solve puzzle and submit solution to Advent of Code server
    Submit {
        input: Option<path::PathBuf>,
    },

    /// Write out a template solution module
    Template,
}

pub fn main() -> anyhow::Result<()> {

    let Opt { account, token, year, day, part, command } = Opt::from_args();
    let cache = aoc::cache::Cache::new(account)?;
    let client = aoc::api::Client::new(cache, &token)?;

    match (command, part) {
    | (Command::Solve { .. }, None) => {
        return Err(anyhow!("[USAGE ERROR]: subcommand `solve` requires flag `--part`"));
    }
    | (Command::Submit { .. }, None)=> {
        return Err(anyhow!("[USAGE ERROR]: subcommand `submit` requires flag `--part`"));
    }

    | (Command::Description, Some(part)) => {
        println!("{}", client.description(year, day, part)?);
    }
    | (Command::Description, None) => {
        println!("{}", client.description(year, day, aoc_core::Part::P01)?);
        println!("{}", client.description(year, day, aoc_core::Part::P02)?);
    }
    | (Command::Input, _) => {
        println!("{}", client.input(year, day)?);
    }
    | (Command::Solve { input }, Some(part)) => {
        let input = match input {
        | Some(path) => read(&path)?,
        | None => client.input(year, day)?,
        };

        println!("{}", solve(year, day, part, &input));
    }
    | (Command::Submit { input }, Some(part)) => {
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
    | (Command::Template, _) => {

        let title = client
            .description(year, day, aoc_core::Part::P01)
            .map(title)?;

        println!("{}", title);

    }
    }

    Ok(())
}

fn read(path: &path::Path) -> anyhow::Result<String> {
    fs::read_to_string(&path)
        .with_context(|| anyhow!("Could not read file: '{}'", path.display()))
}

fn title(description: String) -> String {
    description
        .chars()
        .skip_while(|char| *char != ':')
        .take_while(|char| *char != '\n')
        .scan(true, |capitalize, char| {
            if mem::replace(capitalize, char.is_ascii_whitespace()) {
                Some(char.to_ascii_uppercase())
            } else {
                Some(char)
            }
        })
        .filter(char::is_ascii_alphanumeric)
        .collect()
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
