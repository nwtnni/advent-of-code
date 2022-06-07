use std::cmp;
use std::iter;
use std::num;
use std::ops;

#[derive(thiserror::Error, Clone, Debug)]
pub enum Error {
    #[error("invalid integer: {}", _0)]
    InvalidInt(#[from] num::ParseIntError),

    #[error("invalid year: {}", _0)]
    InvalidYear(String),

    #[error("invalid day: {}", _0)]
    InvalidDay(String),

    #[error("invalid part: {}", _0)]
    InvalidPart(String),

    #[error("invalid digit: {}", _0)]
    InvalidDigit(String),
}

#[macro_export]
macro_rules! pos {
    ($x:expr, $y:expr) => {
        Pos { x: $x, y: $y }
    };
}

#[derive(
    serde::Serialize,
    serde::Deserialize,
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

impl Pos {
    pub fn to_inclusive(self, other: Self) -> impl Iterator<Item = Self> {
        (self.y..=other.y).flat_map(move |y| (self.x..=other.x).map(move |x| Pos { x, y }))
    }

    pub fn to_exclusive(self, other: Self) -> impl Iterator<Item = Self> {
        (self.y..other.y).flat_map(move |y| (self.x..other.x).map(move |x| Pos { x, y }))
    }

    pub fn min(self, other: Pos) -> Self {
        Pos {
            x: cmp::min(self.x, other.x),
            y: cmp::min(self.y, other.y),
        }
    }

    pub fn max(self, other: Pos) -> Self {
        Pos {
            x: cmp::max(self.x, other.x),
            y: cmp::max(self.y, other.y),
        }
    }

    pub fn shift(self, dir: Dir) -> Self {
        match dir {
            Dir::N => Pos {
                x: self.x,
                y: self.y - 1,
            },
            Dir::S => Pos {
                x: self.x,
                y: self.y + 1,
            },
            Dir::E => Pos {
                x: self.x + 1,
                y: self.y,
            },
            Dir::W => Pos {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    pub fn shift_mut(&mut self, dir: Dir) {
        *self = self.shift(dir);
    }
}

#[repr(u8)]
#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    N,
    S,
    W,
    E,
}

impl Dir {
    pub fn all() -> impl Iterator<Item = Self> {
        iter::once(Dir::N)
            .chain(iter::once(Dir::S))
            .chain(iter::once(Dir::E))
            .chain(iter::once(Dir::W))
    }

    pub fn flip(&self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
        }
    }

    pub fn flip_mut(&mut self) {
        *self = self.flip();
    }

    pub fn rotate_clockwise(&self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
            Dir::E => Dir::S,
        }
    }

    pub fn rotate_clockwise_mut(&mut self) {
        *self = self.rotate_clockwise();
    }

    pub fn rotate_counterclockwise(&self) -> Self {
        match self {
            Dir::N => Dir::W,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
            Dir::E => Dir::N,
        }
    }

    pub fn rotate_counterclockwise_mut(&mut self) {
        *self = self.rotate_counterclockwise();
    }
}

impl ops::Neg for Dir {
    type Output = Dir;
    fn neg(self) -> Self::Output {
        self.flip()
    }
}

impl ops::Neg for &Dir {
    type Output = Dir;
    fn neg(self) -> Self::Output {
        self.flip()
    }
}

#[repr(u8)]
#[derive(
    serde::Serialize, serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum Digit {
    #[serde(rename = "0")]
    D0 = 0,
    #[serde(rename = "1")]
    D1 = 1,
    #[serde(rename = "2")]
    D2 = 2,
    #[serde(rename = "3")]
    D3 = 3,
    #[serde(rename = "4")]
    D4 = 4,
    #[serde(rename = "5")]
    D5 = 5,
    #[serde(rename = "6")]
    D6 = 6,
    #[serde(rename = "7")]
    D7 = 7,
    #[serde(rename = "8")]
    D8 = 8,
    #[serde(rename = "9")]
    D9 = 9,
}

#[derive(
    serde::Serialize, serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum Year {
    #[serde(rename = "2015")]
    Y15 = 2015,
    #[serde(rename = "2016")]
    Y16 = 2016,
    #[serde(rename = "2017")]
    Y17 = 2017,
    #[serde(rename = "2018")]
    Y18 = 2018,
    #[serde(rename = "2019")]
    Y19 = 2019,
    #[serde(rename = "2020")]
    Y20 = 2020,
    #[serde(rename = "2021")]
    Y21 = 2021,
}

#[derive(
    serde::Serialize, serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum Day {
    #[serde(rename = "1")]
    D01 = 1,
    #[serde(rename = "2")]
    D02 = 2,
    #[serde(rename = "3")]
    D03 = 3,
    #[serde(rename = "4")]
    D04 = 4,
    #[serde(rename = "5")]
    D05 = 5,
    #[serde(rename = "6")]
    D06 = 6,
    #[serde(rename = "7")]
    D07 = 7,
    #[serde(rename = "8")]
    D08 = 8,
    #[serde(rename = "9")]
    D09 = 9,
    #[serde(rename = "10")]
    D10 = 10,
    #[serde(rename = "11")]
    D11 = 11,
    #[serde(rename = "12")]
    D12 = 12,
    #[serde(rename = "13")]
    D13 = 13,
    #[serde(rename = "14")]
    D14 = 14,
    #[serde(rename = "15")]
    D15 = 15,
    #[serde(rename = "16")]
    D16 = 16,
    #[serde(rename = "17")]
    D17 = 17,
    #[serde(rename = "18")]
    D18 = 18,
    #[serde(rename = "19")]
    D19 = 19,
    #[serde(rename = "20")]
    D20 = 20,
    #[serde(rename = "21")]
    D21 = 21,
    #[serde(rename = "22")]
    D22 = 22,
    #[serde(rename = "23")]
    D23 = 23,
    #[serde(rename = "24")]
    D24 = 24,
    #[serde(rename = "25")]
    D25 = 25,
}

#[repr(u8)]
#[derive(
    serde::Serialize, serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum Part {
    #[serde(rename = "1")]
    P01 = 1,
    #[serde(rename = "2")]
    P02 = 2,
}

#[derive(
    serde::Serialize, serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum Or<L, R> {
    L(L),
    R(R),
}

impl<L, R, T> Iterator for Or<L, R>
where
    L: Iterator<Item = T>,
    R: Iterator<Item = T>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Or::L(l) => l.next(),
            Or::R(r) => r.next(),
        }
    }
}
