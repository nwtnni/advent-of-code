use std::cmp;
use std::str;

pub struct IWasToldThereWouldBeNoMath(Vec<Present>);

impl str::FromStr for IWasToldThereWouldBeNoMath {
    type Err = aoc::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_whitespace()
            .map(Present::from_str)
            .collect::<Result<Vec<_>, _>>()
            .map(IWasToldThereWouldBeNoMath)
    }
}

impl aoc::Solution for IWasToldThereWouldBeNoMath {
    fn one(self) -> i64 {
        self.0.iter().map(Present::wrapping).sum()
    }

    fn two(self) -> i64 {
        self.0.iter().map(Present::ribbon).sum()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Present {
    l: i64,
    w: i64,
    h: i64,
}

impl str::FromStr for Present {
    type Err = aoc::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('x')
            .map(i64::from_str)
            .filter_map(Result::ok);
        let l = iter.next().unwrap();
        let w = iter.next().unwrap();
        let h = iter.next().unwrap();
        Ok(Present { l, w, h })
    }
}

impl Present {
    pub fn area(&self) -> i64 {
        2 * self.l * self.w +
        2 * self.w * self.h +
        2 * self.h * self.l
    }

    pub fn volume(&self) -> i64 {
        self.l * self.w * self.h
    }

    pub fn slack(&self) -> i64 {
        cmp::min(self.l * self.w, cmp::min(self.w * self.h, self.h * self.l))
    }

    pub fn wrapping(&self) -> i64 {
        self.area() + self.slack()
    }

    pub fn ribbon(&self) -> i64 {
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
    fn test_1_0() {
        assert_eq!(Present { l: 2, w: 3, h: 4 }.wrapping(), 58)
    }

    #[test]
    fn test_1_1() {
        assert_eq!(Present { l: 1, w: 1, h: 10 }.wrapping(), 43)
    }

    #[test]
    fn test_2_0() {
        assert_eq!(Present { l: 2, w: 3, h: 4 }.ribbon(), 34)
    }

    #[test]
    fn test_2_1() {
        assert_eq!(Present { l: 1, w: 1, h: 10 }.ribbon(), 14)
    }
}
