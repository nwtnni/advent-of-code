use std::fmt;
use std::str;

use crate::*;

impl str::FromStr for Digit {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Digit::*;
        match s.parse::<u8>().map_err(Error::InvalidInt)? {
            0 => Ok(D0),
            1 => Ok(D1),
            2 => Ok(D2),
            3 => Ok(D3),
            4 => Ok(D4),
            5 => Ok(D5),
            6 => Ok(D6),
            7 => Ok(D7),
            8 => Ok(D8),
            9 => Ok(D9),
            _ => Err(Error::InvalidDigit(s.to_string())),
        }
    }
}

impl Digit {
    pub fn from_char(c: char) -> Option<Self> {
        use Digit::*;
        match c {
            '0' => Some(D0),
            '1' => Some(D1),
            '2' => Some(D2),
            '3' => Some(D3),
            '4' => Some(D4),
            '5' => Some(D5),
            '6' => Some(D6),
            '7' => Some(D7),
            '8' => Some(D8),
            '9' => Some(D9),
            _ => None,
        }
    }

    pub fn from_char_unchecked(c: char) -> Self {
        Self::from_char(c).unwrap()
    }

    pub fn from_int(i: i64) -> Option<Self> {
        use Digit::*;
        match i {
            0 => Some(D0),
            1 => Some(D1),
            2 => Some(D2),
            3 => Some(D3),
            4 => Some(D4),
            5 => Some(D5),
            6 => Some(D6),
            7 => Some(D7),
            8 => Some(D8),
            9 => Some(D9),
            _ => None,
        }
    }

    pub fn from_int_unchecked(i: i64) -> Self {
        Self::from_int(i).unwrap()
    }

    pub fn to_static_str(&self) -> &'static str {
        use Digit::*;
        match self {
            D0 => "0",
            D1 => "1",
            D2 => "2",
            D3 => "3",
            D4 => "4",
            D5 => "5",
            D6 => "6",
            D7 => "7",
            D8 => "8",
            D9 => "9",
        }
    }
}

impl From<Digit> for char {
    fn from(digit: Digit) -> Self {
        use Digit::*;
        match digit {
            D0 => '0',
            D1 => '1',
            D2 => '2',
            D3 => '3',
            D4 => '4',
            D5 => '5',
            D6 => '6',
            D7 => '7',
            D8 => '8',
            D9 => '9',
        }
    }
}

impl fmt::Display for Digit {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", char::from(*self))
    }
}

impl str::FromStr for Year {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Year::*;
        match s.trim().parse::<usize>().map_err(Error::InvalidInt)? {
            15 | 2015 => Ok(Y15),
            16 | 2016 => Ok(Y16),
            17 | 2017 => Ok(Y17),
            18 | 2018 => Ok(Y18),
            19 | 2019 => Ok(Y19),
            20 | 2020 => Ok(Y20),
            21 | 2021 => Ok(Y21),
            22 | 2022 => Ok(Y22),
            _ => Err(Error::InvalidYear(s.to_string())),
        }
    }
}

impl fmt::Display for Year {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_static_str())
    }
}

impl Year {
    pub fn to_static_str(&self) -> &'static str {
        use Year::*;
        match self {
            Y15 => "2015",
            Y16 => "2016",
            Y17 => "2017",
            Y18 => "2018",
            Y19 => "2019",
            Y20 => "2020",
            Y21 => "2021",
            Y22 => "2022",
        }
    }
}

impl str::FromStr for Day {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Day::*;
        match s.parse::<u8>().map_err(Error::InvalidInt)? {
            1 => Ok(D01),
            2 => Ok(D02),
            3 => Ok(D03),
            4 => Ok(D04),
            5 => Ok(D05),
            6 => Ok(D06),
            7 => Ok(D07),
            8 => Ok(D08),
            9 => Ok(D09),
            10 => Ok(D10),
            11 => Ok(D11),
            12 => Ok(D12),
            13 => Ok(D13),
            14 => Ok(D14),
            15 => Ok(D15),
            16 => Ok(D16),
            17 => Ok(D17),
            18 => Ok(D18),
            19 => Ok(D19),
            20 => Ok(D20),
            21 => Ok(D21),
            22 => Ok(D22),
            23 => Ok(D23),
            24 => Ok(D24),
            25 => Ok(D25),
            _ => Err(Error::InvalidDay(s.to_string())),
        }
    }
}

impl fmt::Display for Day {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_static_str())
    }
}

impl Day {
    pub fn to_static_str(&self) -> &'static str {
        use Day::*;
        match self {
            D01 => "1",
            D02 => "2",
            D03 => "3",
            D04 => "4",
            D05 => "5",
            D06 => "6",
            D07 => "7",
            D08 => "8",
            D09 => "9",
            D10 => "10",
            D11 => "11",
            D12 => "12",
            D13 => "13",
            D14 => "14",
            D15 => "15",
            D16 => "16",
            D17 => "17",
            D18 => "18",
            D19 => "19",
            D20 => "20",
            D21 => "21",
            D22 => "22",
            D23 => "23",
            D24 => "24",
            D25 => "25",
        }
    }
}

impl str::FromStr for Part {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Part::*;
        match s.parse::<u8>().map_err(Error::InvalidInt)? {
            1 => Ok(P01),
            2 => Ok(P02),
            _ => Err(Error::InvalidPart(s.to_string())),
        }
    }
}

impl fmt::Display for Part {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_static_str())
    }
}

impl Part {
    pub fn to_static_str(&self) -> &'static str {
        use Part::*;
        match self {
            P01 => "1",
            P02 => "2",
        }
    }
}
