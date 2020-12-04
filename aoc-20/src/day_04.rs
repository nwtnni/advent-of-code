use std::mem;

use aoc::*;

pub struct PassportProcessing(Vec<Vec<String>>);

impl Fro for PassportProcessing {
    fn fro(input: &str) -> Self {
        let mut passports = Vec::new();
        let mut passport = Vec::new();

        input
            .trim()
            .split('\n')
            .for_each(|line| {
                if line.is_empty() {
                    passports.push(mem::take(&mut passport));
                } else {
                    passport.push(line.to_owned());
                }
            });

        passports.push(passport);
        Self(passports)
    }
}

impl Solution for PassportProcessing {
    fn one(self) -> i64 {
        let mut count = 0;
        for passport in self.0 {
            let passport = passport
                .iter()
                .fold(String::new(), |a, b| format!("{} {}", a, b));

            if passport.contains("byr:")
            && passport.contains("iyr:")
            && passport.contains("eyr:")
            && passport.contains("hgt:")
            && passport.contains("hcl:")
            && passport.contains("ecl:")
            && passport.contains("pid:") {
                count += 1;
            }
        }
        count
    }

    fn two(self) -> i64 {
        let mut count = 0;
        for passport in self.0 {
            let mut valid = true;

            let lines = passport.iter().fold(String::new(), |a, b| format!("{} {}", a, b));
            let mut _byr = 0;
            let mut _iyr = 0;
            let mut _eyr = 0;
            let mut _hgt = 0;
            let mut _hcl = 0;
            let mut _ecl = 0;
            let mut _pid = 0;

            for entry in lines.trim().split_whitespace() {
                if entry.starts_with("byr:") {
                    _byr += 1;
                    let byr = entry.split(':').nth(1).unwrap().to::<usize>();
                    valid &= byr >= 1920 && byr <= 2002;
                } else if entry.starts_with("iyr:") {
                    _iyr += 1;
                    let iyr = entry.split(':').nth(1).unwrap().to::<usize>();
                    valid &= iyr >= 2010 && iyr <= 2020;
                } else if entry.starts_with("eyr:") {
                    _eyr += 1;
                    let eyr = entry.split(':').nth(1).unwrap().to::<usize>();
                    valid &= eyr >= 2020 && eyr <= 2030;
                } else if entry.starts_with("hgt:") {

                    _hgt += 1;
                    let hgt = entry.split(':').nth(1).unwrap();

                    if hgt.ends_with("cm") {
                        let val = hgt[0..hgt.len() - 2].to::<usize>();
                        valid &= val >= 150 && val <= 193;
                    } else if hgt.ends_with("in") {
                        let val = hgt[0..hgt.len() - 2].to::<usize>();
                        valid &= val >= 59 && val <= 76;
                    } else {
                        valid = false;
                    }

                } else if entry.starts_with("hcl:") {
                    _hcl += 1;
                    let hcl = entry.split(':').nth(1).unwrap();
                    valid &= hcl.len() == 7
                        && hcl.starts_with("#")
                        && hcl.chars().skip(1).all(|c| c.is_ascii_digit() || (c >= 'a' && c <= 'f'));
                } else if entry.starts_with("ecl:") {
                    _ecl += 1;
                    let ecl = entry.split(':').nth(1).unwrap();
                    valid &= ecl == "amb"
                        || ecl == "blu"
                        || ecl == "brn"
                        || ecl == "gry"
                        || ecl == "grn"
                        || ecl == "hzl"
                        || ecl == "oth";
                } else if entry.starts_with("pid:") {
                    _pid += 1;
                    let pid = entry.split(':').nth(1).unwrap();
                    valid &= pid.len() == 9
                        && pid.chars().all(|c| c.is_ascii_digit());
                }
            }

            if valid
            && _byr == 1
            && _iyr == 1
            && _eyr == 1
            && _hgt == 1
            && _hcl == 1
            && _ecl == 1
            && _pid == 1 {
                count += 1;
            }
        }
        count
    }
}
