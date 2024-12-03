use std::cmp;
use std::cmp::Reverse;
use std::fmt::Display;
use std::fs;
use std::mem;
use std::path::Path;
use std::path::PathBuf;
use std::process;

use anyhow::anyhow;
use anyhow::Context as _;
use aoc::api;
use aoc_core::Day;
use aoc_core::Part;
use aoc_core::Tap as _;
use aoc_core::Year;
use chrono::TimeZone as _;
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
    #[structopt(short, long, env = "AOC_ACCOUNT_ID", hide_env_values = true)]
    id: aoc::leaderboard::Id,

    /// Advent of Code session token
    ///
    /// https://github.com/wimglenn/advent-of-code-wim/issues/1
    #[structopt(
        short,
        long = "session-token",
        env = "AOC_SESSION_TOKEN",
        hide_env_values = true
    )]
    token: String,

    #[structopt(subcommand)]
    command: Command,
}

#[derive(Clone, Debug, StructOpt)]
/// Solve puzzle and print solution
struct Solve {
    #[structopt(short, long)]
    year: Year,

    #[structopt(short, long)]
    day: Day,

    #[structopt(short, long)]
    part: Part,

    /// Path to alternative input text file
    input: Option<PathBuf>,
}

#[derive(Clone, Debug, StructOpt)]
enum Command {
    /// Fetch puzzle description
    Description {
        #[structopt(short, long)]
        year: Year,

        #[structopt(short, long)]
        day: Day,

        #[structopt(short, long)]
        part: Option<Part>,
    },

    /// Download part one description and input, and template out a dummy solution file
    Init {
        #[structopt(short, long)]
        year: Year,

        #[structopt(short, long)]
        day: Day,
    },

    /// Fetch puzzle input
    Input {
        #[structopt(short, long)]
        year: Year,

        #[structopt(short, long)]
        day: Day,
    },

    /// Fetch leaderboard
    Leaderboard {
        #[structopt(short, long)]
        year: Year,

        #[structopt(short, long)]
        day: Day,

        #[structopt(short, long)]
        part: Option<Part>,

        /// Path to alternative leaderboard JSON file
        leaderboard: Option<PathBuf>,
    },

    Solve(Solve),

    /// Solve puzzle and submit solution to Advent of Code server
    Submit {
        #[structopt(short, long)]
        year: Year,

        #[structopt(short, long)]
        day: Day,

        #[structopt(short, long)]
        part: Part,

        /// Alternative answer to submit
        output: Option<i64>,
    },
}

pub fn main() -> anyhow::Result<()> {
    env_logger::init();

    let Opt { id, token, command } = Opt::from_args();
    let client = aoc::api::Client::new(id, &token)?;

    match command {
        Command::Description {
            year,
            day,
            part: Some(part),
        } => {
            println!("{}", client.description(year, day, part)?);
        }
        Command::Description {
            year,
            day,
            part: None,
        } => {
            println!(
                "{}\n\n{}",
                client.description(year, day, aoc_core::Part::P01)?,
                client.description(year, day, aoc_core::Part::P02)?,
            );
        }
        Command::Init { year, day } => {
            let description = client.description(year, day, Part::P01)?;
            let input = client.input(year, day)?;
            let title = title(&description);

            write("description.md", description)?;
            write("input.txt", input)?;

            // Template out Rust code, avoiding clobbering
            let root = PathBuf::from(format!("aoc-{}/src", &year.to_static_str()[2..]));

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
                if index == (day as usize - 1) + 2 {
                    out.push_str(&format!("mod day_{:02};\n", day as usize,));
                }
                if index == ((day as usize - 1) * 2) + 10 {
                    out.push_str(&format!(
                        "        Day::D{day:02} => run!(day_{day:02}::{title}),\n",
                        day = day as usize,
                        title = title,
                    ));
                }
                out.push_str(line);
                out.push('\n');
            }

            write(lib, out)?;
        }
        Command::Input { year, day } => {
            println!("{}", client.input(year, day)?);
        }
        Command::Leaderboard {
            year,
            day,
            part,
            leaderboard,
        } => {
            let leaderboard = match leaderboard {
                Some(path) => read(path)?
                    .tap(|string| json::from_str(&string))
                    .expect("[USAGE]: invalid leaderboard file"),
                None => client.leaderboard(year)?,
            };

            // 12AM ET in UTC
            let start = chrono::Utc
                .with_ymd_and_hms(year as i32, 12, day as u32, 5, 0, 0)
                .unwrap();

            let members = leaderboard
                .members
                .into_iter()
                .filter_map(|(_, member)| {
                    let id = member.id.0;
                    let name = member
                        .name
                        .unwrap_or_else(|| format!("anonymous user #{}", id));
                    let day = member.completion_day_level.get(&day)?;
                    Some((
                        name,
                        day.one.get_star_ts,
                        day.two.map(|day| day.get_star_ts),
                    ))
                })
                .collect::<Vec<_>>()
                .tap_mut(|members| match part {
                    Some(Part::P01) => members.sort_by_key(|(_, first, _)| *first),
                    Some(Part::P02) | None => {
                        // Hack to sort Option::None last.
                        members.sort_by_key(|(_, _, second)| Reverse(second.map(Reverse)))
                    }
                });

            let width = members
                .iter()
                .map(|(name, _, _)| name.len())
                .fold(8, cmp::max);

            print!(
                "Rank | {:<width$} | Part {}",
                "Username",
                part.unwrap_or(Part::P01) as usize,
                width = width,
            );
            println!(
                "{}",
                if part.is_none() {
                    "   | Part 2   | Delta"
                } else {
                    ""
                }
            );

            print!("-----+-{:-<width$}-+---------", "-", width = width);
            println!(
                "{}",
                if part.is_none() {
                    "-+----------+---------"
                } else {
                    ""
                }
            );

            for (index, (name, first, second)) in members.iter().enumerate() {
                print!(
                    "{:02}   | {:<width$} | {}",
                    index,
                    name,
                    Duration(
                        match part {
                            Some(Part::P01) | None => first,
                            Some(Part::P02) => match second {
                                None => continue,
                                Some(second) => second,
                            },
                        }
                        .signed_duration_since(start)
                    ),
                    width = width,
                );

                if part.is_some() {
                    println!();
                    continue;
                }

                let Some(second) = second else {
                    println!(" |");
                    continue;
                };

                println!(
                    " | {} | {}",
                    Duration(second.signed_duration_since(start)),
                    Duration(second.signed_duration_since(first)),
                );
            }
        }
        Command::Solve(Solve {
            year,
            day,
            part,
            input,
        }) => {
            let input = input.map(read).unwrap_or_else(|| client.input(year, day))?;

            println!("{}", solve(year, day, part, &input));
        }
        Command::Submit {
            year,
            day,
            part,
            output,
        } => {
            let output = match output {
                Some(output) => output,
                None => client
                    .input(year, day)?
                    .tap(|input| solve(year, day, part, &input)),
            };

            match client.submit(year, day, part, output)? {
                api::Response::Correct => log::info!("{} was correct!", output),
                response => {
                    log::info!("{} was incorrect: {:?}", output, response);
                    process::exit(1);
                }
            }

            if part == Part::P02 {
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

fn read<P: AsRef<Path>>(path: P) -> anyhow::Result<String> {
    let path = path.as_ref();
    fs::read_to_string(path).with_context(|| anyhow!("Could not read file: '{}'", path.display()))
}

fn write<P, D>(path: P, data: D) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    D: AsRef<[u8]>,
{
    let path = path.as_ref();
    let data = data.as_ref();

    log::info!("Writing to {}", path.display());

    fs::write(path, data).with_context(|| anyhow!("Could not write to file: '{}'", path.display()))
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

fn solve(year: Year, day: Day, part: Part, input: &str) -> i64 {
    match year {
        Year::Y15 => aoc_15::solve(day, part, input),
        Year::Y16 => aoc_16::solve(day, part, input),
        Year::Y17 => aoc_17::solve(day, part, input),
        Year::Y18 => aoc_18::solve(day, part, input),
        Year::Y19 => aoc_19::solve(day, part, input),
        Year::Y20 => aoc_20::solve(day, part, input),
        Year::Y21 => aoc_21::solve(day, part, input),
        Year::Y22 => aoc_22::solve(day, part, input),
        Year::Y23 => aoc_23::solve(day, part, input),
        Year::Y24 => aoc_24::solve(day, part, input),
    }
}

struct Duration(chrono::Duration);

impl Display for Duration {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "{:02}:{:02}:{:02}",
            self.0.num_hours(),
            self.0.num_minutes() % 60,
            self.0.num_seconds() % 60,
        )
    }
}
