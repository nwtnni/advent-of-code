use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
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
            .count() as i64
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
                    && hcl
                        .chars()
                        .skip(1)
                        .all(|c| c.is_ascii_digit() || (c >= 'a' && c <= 'f'))
            })
            .filter(|passport| {
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&*passport["ecl"])
            })
            .filter(|passport| {
                let pid = &passport["pid"];
                pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit())
            })
            .count() as i64
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

#[cfg(test)]
mod tests {

    use aoc::Fro as _;
    use aoc::Solution as _;

    static EXAMPLE_ONE: &str = "
        ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
        byr:1937 iyr:2017 cid:147 hgt:183cm\n\
        \n\
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
        hcl:#cfa07d byr:1929\n\
        \n\
        hcl:#ae17e1 iyr:2013\n\
        eyr:2024\n\
        ecl:brn pid:760753108 byr:1931\n\
        hgt:179cm\n\
        \n\
        hcl:#cfa07d eyr:2025 pid:166559648\n\
        iyr:2011 ecl:brn hgt:59in\
    ";

    #[test]
    fn part_one() {
        assert_eq!(super::PassportProcessing::fro(EXAMPLE_ONE).one(), 2);
    }

    static EXAMPLE_TWO: &str = "
        eyr:1972 cid:100\n\
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
        \n\
        iyr:2019\n\
        hcl:#602927 eyr:1967 hgt:170cm\n\
        ecl:grn pid:012533040 byr:1946\n\
        \n\
        hcl:dab227 iyr:2012\n\
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
        \n\
        hgt:59cm ecl:zzz\n\
        eyr:2038 hcl:74454a iyr:2023\n\
        pid:3556412378 byr:2007\n\
        \n\
        pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
        hcl:#623a2f\n\
        \n\
        eyr:2029 ecl:blu cid:129 byr:1989\n\
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
        \n\
        hcl:#888785\n\
        hgt:164cm byr:2001 iyr:2015 cid:88\n\
        pid:545766238 ecl:hzl\n\
        eyr:2022\n\
        \n\
        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719\
    ";

    #[test]
    fn part_two() {
        assert_eq!(super::PassportProcessing::fro(EXAMPLE_TWO).two(), 4);
    }
}
