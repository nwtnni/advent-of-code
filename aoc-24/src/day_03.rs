use std::sync::OnceLock;

use aoc::*;
use regex::Regex;

static MUL: OnceLock<Regex> = OnceLock::new();
static DISABLE: OnceLock<Regex> = OnceLock::new();

fn mul() -> &'static Regex {
    MUL.get_or_init(|| Regex::new(r#"mul\(([[:digit:]]{1,3}),([[:digit:]]{1,3})\)"#).unwrap())
}

// Need s flag for `.` to match newline :(
// https://docs.rs/regex/latest/regex/index.html#grouping-and-flags
fn disable() -> &'static Regex {
    DISABLE.get_or_init(|| Regex::new(r#"don't\(\)(?s:.)*?(?:(?:do\(\))|$)"#).unwrap())
}

#[derive(Clone, Debug)]
pub struct MullItOver(String);

impl Fro for MullItOver {
    fn fro(input: &str) -> Self {
        Self(input.trim().to_owned())
    }
}

impl Solution for MullItOver {
    fn one(self) -> i64 {
        mul()
            .captures_iter(&self.0)
            .map(|capture| (capture[1].to::<i64>(), capture[2].to::<i64>()))
            .map(|(l, r)| l * r)
            .sum()
    }

    fn two(self) -> i64 {
        Self(disable().replace_all(&self.0, "").to_string()).one()
    }
}
