use std::cmp;
use std::cmp::Ordering;
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
    pub fn i(&self) -> i64 {
        self.y
    }

    pub fn j(&self) -> i64 {
        self.x
    }

    pub fn from_i_j(i: i64, j: i64) -> Self {
        Pos { x: j, y: i }
    }

    pub fn from_index(cols: i64, index: usize) -> Self {
        Pos {
            x: index as i64 % cols,
            y: index as i64 / cols,
        }
    }

    pub fn to_index(&self, cols: i64) -> usize {
        (self.y * cols + self.x) as usize
    }

    pub fn border_inclusive(self, other: Self) -> impl Iterator<Item = Self> {
        (self.y..=other.y).flat_map(move |y| {
            if y == self.y || y == other.y {
                Or::L(Or::L(self.x..=other.x))
            } else {
                match self.x.cmp(&other.x) {
                    Ordering::Less => Or::L(Or::R(IntoIterator::into_iter([self.x, other.x]))),
                    Ordering::Equal => Or::R(Or::L(iter::once(self.x))),
                    Ordering::Greater => Or::R(Or::R(iter::empty())),
                }
            }
            .map(move |x| Pos { x, y })
        })
    }

    pub fn border_exclusive(self, other: Self) -> impl Iterator<Item = Self> {
        (self.y..other.y).flat_map(move |y| {
            if y == self.y || y + 1 == other.y {
                Or::L(Or::L(self.x..other.x))
            } else {
                match self.x.cmp(&(other.x - 1)) {
                    Ordering::Less => Or::L(Or::R(IntoIterator::into_iter([self.x, other.x - 1]))),
                    Ordering::Equal => Or::R(Or::L(iter::once(self.x))),
                    Ordering::Greater => Or::R(Or::R(iter::empty())),
                }
            }
            .map(move |x| Pos { x, y })
        })
    }

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

    pub fn distance(self, other: Pos) -> i64 {
        self.x.abs_diff(other.x).max(self.y.abs_diff(other.y)) as i64
    }

    pub fn distance_manhattan(self, other: Pos) -> i64 {
        self.x.abs_diff(other.x) as i64 + self.y.abs_diff(other.y) as i64
    }

    /// All points within `radius` of `self`, including `self`.
    pub fn around_inclusive(&self, radius: i64) -> impl Iterator<Item = Self> {
        let center = *self;
        ((center.y - radius)..=(center.y + radius)).flat_map(move |y| {
            ((center.x - radius)..=(center.x + radius)).map(move |x| Pos { x, y })
        })
    }

    /// All points within `radius` of `self`, excluding `self`.
    pub fn around_exclusive(&self, radius: i64) -> impl Iterator<Item = Self> {
        let center = *self;
        self.around_inclusive(radius)
            .filter(move |other| *other != center)
    }

    /// All points at exactly `radius` from `self`.
    pub fn around_exact(&self, radius: i64) -> impl Iterator<Item = Self> {
        let center = *self;
        let upper = center.y - radius;
        let lower = center.y + radius;
        ((center.y - radius)..=(center.y + radius)).flat_map(move |y| {
            match y == upper || y == lower {
                true => Or::L((center.x - radius)..=(center.x + radius)),
                false => Or::R(IntoIterator::into_iter([
                    center.x - radius,
                    center.x + radius,
                ])),
            }
            .map(move |x| Pos { x, y })
        })
    }

    /// All points within `radius` of `self`, including `self`.
    pub fn around_manhattan_inclusive(&self, radius: i64) -> impl Iterator<Item = Self> {
        let center = *self;
        ((center.y - radius)..=(center.y + radius)).flat_map(move |y| {
            let radius = radius - y.abs_diff(center.y) as i64;
            ((center.x - radius)..=(center.x + radius)).map(move |x| Pos { x, y })
        })
    }

    /// All points within `radius` of `self`, excluding `self`.
    pub fn around_manhattan_exclusive(&self, radius: i64) -> impl Iterator<Item = Self> {
        let center = *self;
        self.around_manhattan_inclusive(radius)
            .filter(move |other| *other != center)
    }

    /// All points at exactly `radius` from `self`.
    pub fn around_manhattan_exact(&self, radius: i64) -> impl Iterator<Item = Self> {
        let center = *self;
        ((center.y - radius)..=(center.y + radius)).flat_map(move |y| {
            let radius = radius - y.abs_diff(center.y) as i64;
            match radius {
                0 => Or::L(std::iter::once(Pos { x: center.x, y })),
                _ => Or::R(
                    IntoIterator::into_iter([center.x - radius, center.x + radius])
                        .map(move |x| Pos { x, y }),
                ),
            }
        })
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

    pub fn shiftn(self, dir: Dir, n: i64) -> Self {
        match dir {
            Dir::N => Pos {
                x: self.x,
                y: self.y - n,
            },
            Dir::S => Pos {
                x: self.x,
                y: self.y + n,
            },
            Dir::E => Pos {
                x: self.x + n,
                y: self.y,
            },
            Dir::W => Pos {
                x: self.x - n,
                y: self.y,
            },
        }
    }

    pub fn shiftn_mut(&mut self, dir: Dir, n: i64) {
        *self = self.shiftn(dir, n);
    }
}

impl std::ops::Sub for Pos {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Add for Pos {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Neg for Pos {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Pos {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::ops::Mul<i64> for Pos {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Pos {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::MulAssign<i64> for Pos {
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs;
    }
}

impl std::ops::Div<i64> for Pos {
    type Output = Self;
    fn div(self, rhs: i64) -> Self::Output {
        Pos {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::DivAssign<i64> for Pos {
    fn div_assign(&mut self, rhs: i64) {
        *self = *self / rhs;
    }
}

#[repr(u8)]
#[derive(
    serde::Serialize, serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
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

    pub fn rotate(&self, clockwise: bool) -> Self {
        match clockwise {
            true => self.rotate_clockwise(),
            false => self.rotate_counterclockwise(),
        }
    }

    pub fn rotate_mut(&mut self, clockwise: bool) {
        match clockwise {
            true => self.rotate_clockwise_mut(),
            false => self.rotate_counterclockwise_mut(),
        }
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
    #[serde(rename = "2022")]
    Y22 = 2022,
    #[serde(rename = "2023")]
    Y23 = 2023,
    #[serde(rename = "2024")]
    Y24 = 2024,
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
