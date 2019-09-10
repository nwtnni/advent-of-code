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
    
    #[fail(display = "invalid digit: {}", _0)]
    InvalidDigit(String),
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Digit {
    D0 = 0,
    D1 = 1,
    D2 = 2,
    D3 = 3,
    D4 = 4,
    D5 = 5,
    D6 = 6,
    D7 = 7,
    D8 = 8,
    D9 = 9,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Part {
    P01 = 1,
    P02 = 2,
}

pub trait Solution {
    fn one(&mut self) -> usize;
    fn two(&mut self) -> usize;
    fn solve(&mut self, part: Part) -> usize {
        match part {
        | Part::P01 => self.one(),
        | Part::P02 => self.two(),
        }
    }
}
