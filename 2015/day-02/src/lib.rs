use std::str::FromStr;
use std::usize;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Present {
    l: usize,
    w: usize,
    h: usize,
}

impl Present {
    pub fn parse(dimensions: &str) -> Self {
        let mut iter = dimensions.split('x')
            .map(usize::from_str)
            .filter_map(Result::ok);
        let l = iter.next().unwrap();
        let w = iter.next().unwrap();
        let h = iter.next().unwrap();
        Present { l, w, h }
    }

    pub fn area(self) -> usize {
        2 * self.l * self.w +
        2 * self.w * self.h +
        2 * self.h * self.l
    }

    pub fn volume(self) -> usize {
        self.l * self.w * self.h
    }

    pub fn slack(self) -> usize {
        usize::min(self.l * self.w, usize::min(self.w * self.h, self.h * self.l))
    }

    pub fn wrapping(self) -> usize {
        self.area() + self.slack()
    }

    pub fn ribbon(self) -> usize {
        let lw = 2 * (self.l + self.w);
        let wh = 2 * (self.w + self.h);
        let hl = 2 * (self.h + self.l);
        let around = usize::min(lw, usize::min(wh, hl));
        around + self.volume()
    }
}
