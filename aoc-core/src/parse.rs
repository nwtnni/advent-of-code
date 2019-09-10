use std::str;

impl str::FromStr for crate::Digit {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::Digit::*;
        match s.parse::<u8>().map_err(crate::Error::InvalidInt)? {
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
        | _ => Err(crate::Error::InvalidDigit(s.to_string()))
        }
    }
}

impl str::FromStr for crate::Year {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::Year::*;
        match s.parse::<u8>().map_err(crate::Error::InvalidInt)? {
        | 15 => Ok(Y15),
        | 17 => Ok(Y17),
        | 18 => Ok(Y18),
        | _ => Err(crate::Error::InvalidPart(s.to_string())),
        }
    }
}


impl str::FromStr for crate::Day {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::Day::*;
        match s.parse::<u8>().map_err(crate::Error::InvalidInt)? {
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
        | _ => Err(crate::Error::InvalidDay(s.to_string())),
        }
    }
}

impl str::FromStr for crate::Part {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::Part::*;
        match s.parse::<u8>().map_err(crate::Error::InvalidInt)? {
        | 1 => Ok(P01),
        | 2 => Ok(P02),
        | _ => Err(crate::Error::InvalidPart(s.to_string())),
        }
    }
}
