use std::num;
use std::str;

#[derive(Debug, failure::Fail)]
pub enum Error {
    #[fail(display = "invalid integer: {}", _0)]
    InvalidInt(#[fail(cause)] num::ParseIntError),

    #[fail(display = "invalid day: {}", _0)]
    InvalidDay(String),

    #[fail(display = "invalid part: {}", _0)]
    InvalidPart(String),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Day {
    D01 = 01, 
    D02 = 02, 
    D03 = 03, 
    D04 = 04, 
    D05 = 05, 
    D06 = 06, 
    D07 = 07, 
    D08 = 08, 
    D09 = 09, 
    D10 = 10, 
    D11 = 11, 
    D12 = 12, 
    D13 = 13, 
    D14 = 14, 
    D15 = 15, 
    D16 = 16, 
    D17 = 17, 
    D18 = 18, 
    D19 = 19, 
    D20 = 20, 
    D21 = 21, 
    D22 = 22, 
    D23 = 23, 
    D24 = 24, 
    D25 = 25, 
}

impl str::FromStr for Day {
    type Err = failure::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let day = match s.parse::<u8>().map_err(Error::InvalidInt)? {
        | 01 => Day::D01,
        | 02 => Day::D02,
        | 03 => Day::D03,
        | 04 => Day::D04,
        | 05 => Day::D05,
        | 06 => Day::D06,
        | 07 => Day::D07,
        | 08 => Day::D08,
        | 09 => Day::D09,
        | 10 => Day::D10,
        | 11 => Day::D11,
        | 12 => Day::D12,
        | 13 => Day::D13,
        | 14 => Day::D14,
        | 15 => Day::D15,
        | 16 => Day::D16,
        | 17 => Day::D17,
        | 18 => Day::D18,
        | 19 => Day::D19,
        | 20 => Day::D20,
        | 21 => Day::D21,
        | 22 => Day::D22,
        | 23 => Day::D23,
        | 24 => Day::D24,
        | 25 => Day::D25,
        | _ => Err(Error::InvalidDay(s.to_string()))?,
        };
        Ok(day)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Part {
    P01 = 1,
    P02 = 2,
}

pub trait Solution: str::FromStr {
    fn one(self) -> usize;
    fn two(self) -> usize;
    fn solve(self, part: Part) -> usize {
        match part {
        | Part::P01 => self.one(),
        | Part::P02 => self.two(),
        }
    }
}
