use std::iter;
use std::num;
use std::ops;

#[derive(thiserror::Error)]
#[derive(Clone, Debug)]
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
        Pos {
            x: $x,
            y: $y,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

impl Pos {
    pub fn shift(&self, dir: Dir) -> Self {
        match dir {
        | Dir::N => Pos { x: self.x, y: self.y - 1 },
        | Dir::S => Pos { x: self.x, y: self.y + 1 },
        | Dir::E => Pos { x: self.x + 1, y: self.y },
        | Dir::W => Pos { x: self.x - 1, y: self.y },
        }
    }

    pub fn shift_mut(&mut self, dir: Dir) {
        *self = self.shift(dir);
    }
}

#[repr(u8)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Dir { N, S, W, E }

impl Dir {
    pub fn all() -> impl Iterator<Item = Self> {
        iter::once(Dir::N)
            .chain(iter::once(Dir::S))
            .chain(iter::once(Dir::E))
            .chain(iter::once(Dir::W))
    }

    pub fn flip(&self) -> Dir {
        match self {
        | Dir::N => Dir::S,
        | Dir::S => Dir::N,
        | Dir::E => Dir::W,
        | Dir::W => Dir::E,
        }
    }

    pub fn flip_mut(&mut self) {
        *self = self.flip();
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
#[derive(serde::Serialize, serde::Deserialize)]
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
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Year {
    Y15 = 15,
    Y17 = 17,
    Y18 = 18,
    Y19 = 19,
}

#[repr(u8)]
#[derive(serde::Serialize, serde::Deserialize)]
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
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Part {
    P01 = 1,
    P02 = 2,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Or<L, R> {
    L(L),
    R(R),
}

impl<L, R, T> Iterator for Or<L, R>
where L: Iterator<Item = T>,
      R: Iterator<Item = T>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
        | Or::L(l) => l.next(),
        | Or::R(r) => r.next(),
        }
    }
}
