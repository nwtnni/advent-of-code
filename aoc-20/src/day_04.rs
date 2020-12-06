use std::collections::HashMap;

use aoc::*;

pub struct PassportProcessing(Vec<HashMap<String, String>>);

impl Fro for PassportProcessing {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split("\n\n")
            .map(|line| {
                line.split_whitespace()
                    .map(|entry| {
                        let mut iter = entry.split(':');
                        (iter.give().to_owned(), iter.give().to_owned())
                    })
                    .collect()
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for PassportProcessing {
    fn one(self) -> i64 {
        self.0
            .into_iter()
            .filter(|passport| required(&passport))
            .count()
            as i64
    }

    fn two(self) -> i64 {
        self.0
            .into_iter()
            .filter(|passport| required(&passport))
            .filter(|passport| between(&passport["byr"], 1920, 2002))
            .filter(|passport| between(&passport["iyr"], 2010, 2020))
            .filter(|passport| between(&passport["eyr"], 2020, 2030))
            .filter(|passport| {
                let hgt = &passport["hgt"];
                if hgt.ends_with("cm") {
                    between(&hgt[0..hgt.len() - 2], 150, 193)
                } else if hgt.ends_with("in") {
                    between(&hgt[0..hgt.len() - 2], 59, 76)
                } else {
                    false
                }
            })
            .filter(|passport| {
                let hcl = &passport["hcl"];
                hcl.len() == 7
                    && hcl.starts_with("#")
                    && hcl.chars().skip(1).all(|c| c.is_ascii_digit() || (c >= 'a' && c <= 'f'))
            })
            .filter(|passport| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&*passport["ecl"]))
            .filter(|passport| {
                let pid = &passport["pid"];
                pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit())
            })
            .count()
            as i64
    }
}

fn required(passport: &HashMap<String, String>) -> bool {
    passport.contains_key("byr")
        && passport.contains_key("iyr")
        && passport.contains_key("eyr")
        && passport.contains_key("hgt")
        && passport.contains_key("hcl")
        && passport.contains_key("ecl")
        && passport.contains_key("pid")
}

fn between(string: &str, lo: usize, hi: usize) -> bool {
    let value = string.parse::<usize>().unwrap();
    value >= lo && value <= hi
}
