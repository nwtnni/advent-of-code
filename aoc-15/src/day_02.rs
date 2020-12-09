use std::cmp;

use aoc::*;

#[derive(Clone, Debug)]
pub struct IWasToldThereWouldBeNoMath(Vec<Present>);

impl Fro for IWasToldThereWouldBeNoMath {
    fn fro(input: &str) -> Self {
        input
            .split_whitespace()
            .map(Present::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for IWasToldThereWouldBeNoMath {
    fn one(self) -> i64 {
        self.0.into_iter().map(Present::wrapping).sum()
    }

    fn two(self) -> i64 {
        self.0.into_iter().map(Present::ribbon).sum()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Present {
    l: i64,
    w: i64,
    h: i64,
}

impl Fro for Present {
    fn fro(input: &str) -> Self {
        let mut iter = input.split('x').map(i64::fro);
        let l = iter.give();
        let w = iter.give();
        let h = iter.give();
        Present { l, w, h }
    }
}

impl Present {
    pub fn area(self) -> i64 {
        2 * self.l * self.w +
        2 * self.w * self.h +
        2 * self.h * self.l
    }

    pub fn volume(self) -> i64 {
        self.l * self.w * self.h
    }

    pub fn slack(self) -> i64 {
        cmp::min(self.l * self.w, cmp::min(self.w * self.h, self.h * self.l))
    }

    pub fn wrapping(self) -> i64 {
        self.area() + self.slack()
    }

    pub fn ribbon(self) -> i64 {
        let lw = 2 * (self.l + self.w);
        let wh = 2 * (self.w + self.h);
        let hl = 2 * (self.h + self.l);
        let around = cmp::min(lw, cmp::min(wh, hl));
        around + self.volume()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_0() {
        assert_eq!(Present { l: 2, w: 3, h: 4 }.wrapping(), 58)
    }

    #[test]
    fn part_one_1() {
        assert_eq!(Present { l: 1, w: 1, h: 10 }.wrapping(), 43)
    }

    #[test]
    fn part_two_0() {
        assert_eq!(Present { l: 2, w: 3, h: 4 }.ribbon(), 34)
    }

    #[test]
    fn part_two_1() {
        assert_eq!(Present { l: 1, w: 1, h: 10 }.ribbon(), 14)
    }
}
