use std::fs;
use std::mem;
use std::path;
use std::process;

use aoc_core::Tap as _;
use anyhow::anyhow;
use anyhow::Context as _;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "aoc", about = "Advent of Code CLI utility")]
struct Opt {
    /// Advent of Code account ID
    ///
    /// This can be found in the URL of your [private leaderboard][pl]. Used to
    /// correlate cache entries across different session tokens, and to access
    /// the private leaderboard API.
    ///
    /// [pl]: https://adventofcode.com/2020/leaderboard/private
    #[structopt(short, long, env = "AOC_ACCOUNT_ID")]
    id: aoc::leaderboard::Id,

    /// Advent of Code session token
    ///
    /// https://github.com/wimglenn/advent-of-code-wim/issues/1
    #[structopt(short, long = "session-token", env = "AOC_SESSION_TOKEN")]
    token: String,

    #[structopt(subcommand)]
    command: Command,
}

#[derive(Clone, Debug, StructOpt)]
enum Command {
    /// Fetch puzzle description
    Description {
        #[structopt(short, long)]
        year: aoc_core::Year,

        #[structopt(short, long)]
        day: aoc_core::Day,

        #[structopt(short, long)]
        part: Option<aoc_core::Part>,
    },

    /// Download part one description and input, and template out a dummy solution file
    Init {
        #[structopt(short, long)]
        year: aoc_core::Year,

        #[structopt(short, long)]
        day: aoc_core::Day,
    },

    /// Fetch puzzle input
    Input {
        #[structopt(short, long)]
        year: aoc_core::Year,

        #[structopt(short, long)]
        day: aoc_core::Day,
    },

    /// Fetch leaderboard
    Leaderboard {
        #[structopt(short, long)]
        year: aoc_core::Year,
    },

    /// Solve puzzle and print solution
    Solve {
        #[structopt(short, long)]
        year: aoc_core::Year,

        #[structopt(short, long)]
        day: aoc_core::Day,

        #[structopt(short, long)]
        part: aoc_core::Part,

        /// Path to alternative input file
        input: Option<path::PathBuf>,
    },

    /// Solve puzzle and submit solution to Advent of Code server
    Submit {
        #[structopt(short, long)]
        year: aoc_core::Year,

        #[structopt(short, long)]
        day: aoc_core::Day,

        #[structopt(short, long)]
        part: aoc_core::Part,

        /// Alternative answer to submit
        output: Option<i64>,
    },
}

pub fn main() -> anyhow::Result<()> {

    env_logger::init();

    let Opt { id, token, command } = Opt::from_args();
    let client = aoc::api::Client::new(id, &token)?;

    match command {
    | Command::Description { year, day, part: Some(part) } => {
        println!("{}", client.description(year, day, part)?);
    }
    | Command::Description { year, day, part: None } => {
        println!(
            "{}\n\n{}",
            client.description(year, day, aoc_core::Part::P01)?,
            client.description(year, day, aoc_core::Part::P02)?,
        );
    }
    | Command::Init { year, day } => {
        let description = client.description(year, day, aoc_core::Part::P01)?;
        let input = client.input(year, day)?;
        let title = title(&description);

        write("description.md", &description)?;
        write("input.txt", &input)?;

        // Template out Rust code, avoiding clobbering
        let root = path::PathBuf::from(format!("aoc-{}/src", &year.to_static_str()[2..]));

        let r#mod = root.join(format!("day_{:02}.rs", day as usize));
        if !r#mod.exists() {
            write(r#mod, include_str!("template.rs").replace("$TITLE", &title))?;
        } else {
            log::info!("Skipping existing module: {}", r#mod.display());
        }

        let lib = root.join("lib.rs");
        let r#in = read(&lib)?;

        if r#in.contains(&title) {
            log::info!("Skipping updated library: {}", lib.display());
            return Ok(());
        }

        let mut out = String::new();

        for (index, line) in r#in.trim().split('\n').enumerate() {
            if index == ((day as usize - 1) * 1) + 2 {
                out.push_str(&format!(
                    "mod day_{:02};\n",
                    day as usize,
                ));
            }
            if index == ((day as usize - 1) * 2) + 10 {
                out.push_str(&format!(
                    "    | Day::D{day:02} => run!(day_{day:02}::{title}),\n",
                    day = day as usize,
                    title = title,
                ));
            }
            out.push_str(line);
            out.push('\n');
        }

        write(lib, out)?;
    }
    | Command::Input { year, day } => {
        println!("{}", client.input(year, day)?);
    }
    | Command::Leaderboard { year: _ } => {
        todo!()
    }
    | Command::Solve { year, day, part, input } => {
        let input = input
            .map(read)
            .unwrap_or_else(|| client.input(year, day))?;

        println!("{}", solve(year, day, part, &input));
    }
    | Command::Submit { year, day, part, output } => {
        let output = match output {
        | Some(output) => output,
        | None => client
            .input(year, day)?
            .tap(|input| solve(year, day, part, &input)),
        };

        if !client.submit(year, day, part, output)? {
            log::info!("{} was incorrect!", output);
            process::exit(1);
        }

        log::info!("{} was correct!", output);

        if part == aoc_core::Part::P02 {
            return Ok(());
        }

        log::info!("Writing complete description to `description.md`...");
        write(
            "description.md",
            format!(
                "{}\n\n{}",
                client.description(year, day, aoc_core::Part::P01)?,
                client.description(year, day, aoc_core::Part::P02)?,
            ),
        )?;
    }
    }

    Ok(())
}

fn read<P: AsRef<path::Path>>(path: P) -> anyhow::Result<String> {
    let path = path.as_ref();
    fs::read_to_string(path)
        .with_context(|| anyhow!("Could not read file: '{}'", path.display()))
}

fn write<P, D>(path: P, data: D) -> anyhow::Result<()>
where
    P: AsRef<path::Path>,
    D: AsRef<[u8]>,
{
    let path = path.as_ref();
    let data = data.as_ref();

    log::info!("Writing to {}", path.display());

    fs::write(path, data)
        .with_context(|| anyhow!("Could not write to file: '{}'", path.display()))
}

fn title(description: &str) -> String {
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
    | aoc_core::Year::Y20 => aoc_20::solve(day, part, &input),
    | _ => unimplemented!(),
    }
}
