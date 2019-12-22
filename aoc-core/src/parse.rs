use std::str;

use crate::*;

impl str::FromStr for Digit {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Digit::*;
        match s.parse::<u8>().map_err(Error::InvalidInt)? {
        | 0 => Ok(D0),
        | 1 => Ok(D1),
        | 2 => Ok(D2),
        | 3 => Ok(D3),
        | 4 => Ok(D4),
        | 5 => Ok(D5),
        | 6 => Ok(D6),
        | 7 => Ok(D7),
        | 8 => Ok(D8),
        | 9 => Ok(D9),
        | _ => Err(Error::InvalidDigit(s.to_string()))
        }
    }
}

impl Digit {
    pub fn from_char(c: char) -> Option<Self> {
        use Digit::*;
        match c {
        | '0' => Some(D0),
        | '1' => Some(D1),
        | '2' => Some(D2),
        | '3' => Some(D3),
        | '4' => Some(D4),
        | '5' => Some(D5),
        | '6' => Some(D6),
        | '7' => Some(D7),
        | '8' => Some(D8),
        | '9' => Some(D9),
        | _ => None,
        }
    }

    pub fn from_char_unchecked(c: char) -> Self {
        Self::from_char(c).unwrap()
    }

    pub fn from_int(i: i64) -> Option<Self> {
        use Digit::*;
        match i {
        | 0 => Some(D0),
        | 1 => Some(D1),
        | 2 => Some(D2),
        | 3 => Some(D3),
        | 4 => Some(D4),
        | 5 => Some(D5),
        | 6 => Some(D6),
        | 7 => Some(D7),
        | 8 => Some(D8),
        | 9 => Some(D9),
        | _ => None,
        }
    }

    pub fn from_int_unchecked(i: i64) -> Self {
        Self::from_int(i).unwrap()
    }
}

impl From<Digit> for char {
    fn from(digit: Digit) -> Self {
        use Digit::*;
        match digit {
        | D0 => '0',
        | D1 => '1',
        | D2 => '2',
        | D3 => '3',
        | D4 => '4',
        | D5 => '5',
        | D6 => '6',
        | D7 => '7',
        | D8 => '8',
        | D9 => '9',
        }
    }
}

impl str::FromStr for Year {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Year::*;
        match s.parse::<u8>().map_err(Error::InvalidInt)? {
        | 15 => Ok(Y15),
        | 17 => Ok(Y17),
        | 18 => Ok(Y18),
        | 19 => Ok(Y19),
        | _ => Err(Error::InvalidYear(s.to_string())),
        }
    }
}

impl str::FromStr for Day {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Day::*;
        match s.parse::<u8>().map_err(Error::InvalidInt)? {
        | 01 => Ok(D01),
        | 02 => Ok(D02),
        | 03 => Ok(D03),
        | 04 => Ok(D04),
        | 05 => Ok(D05),
        | 06 => Ok(D06),
        | 07 => Ok(D07),
        | 08 => Ok(D08),
        | 09 => Ok(D09),
        | 10 => Ok(D10),
        | 11 => Ok(D11),
        | 12 => Ok(D12),
        | 13 => Ok(D13),
        | 14 => Ok(D14),
        | 15 => Ok(D15),
        | 16 => Ok(D16),
        | 17 => Ok(D17),
        | 18 => Ok(D18),
        | 19 => Ok(D19),
        | 20 => Ok(D20),
        | 21 => Ok(D21),
        | 22 => Ok(D22),
        | 23 => Ok(D23),
        | 24 => Ok(D24),
        | 25 => Ok(D25),
        | _ => Err(Error::InvalidDay(s.to_string())),
        }
    }
}

impl str::FromStr for Part {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Part::*;
        match s.parse::<u8>().map_err(Error::InvalidInt)? {
        | 1 => Ok(P01),
        | 2 => Ok(P02),
        | _ => Err(Error::InvalidPart(s.to_string())),
        }
    }
}
