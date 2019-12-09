use std::fmt::Write;
use std::str;

pub struct TheIdealStockingStuffer {
    buf: String,
    len: usize,
}

impl str::FromStr for TheIdealStockingStuffer {
    type Err = aoc::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            buf: s.trim().to_owned(),
            len: s.trim().len(),
        })
    }
}

impl TheIdealStockingStuffer {
    #[inline(always)]
    fn hash(&mut self, salt: i64) -> md5::Digest {
        self.buf.truncate(self.len);
        write!(self.buf, "{}", salt).ok();
        md5::compute(&self.buf)
    }

    fn find(&mut self, len: usize) -> i64 {
        (1i64..).find(|&salt| {
            let digest = self.hash(salt).0;
            (0..len).all(|i| {
                let byte = digest[i >> 1];
                let mask = 0b1111_0000 >> ((i & 1) << 2);
                byte & mask == 0
            })
        }).unwrap()
    }
}

impl aoc::Solution for TheIdealStockingStuffer {
    fn one(mut self) -> i64 {
        self.find(5)
    }

    fn two(mut self) -> i64 {
        self.find(6)
    }
}

#[cfg(test)]
mod tests {

    use aoc::Solution;

    type ISS = super::TheIdealStockingStuffer;

    #[test]
    fn test_0() {
        assert_eq!("abcdef".parse::<ISS>().unwrap().one(), 609043)
    }

    #[test]
    fn test_1() {
        assert_eq!("pqrstuv".parse::<ISS>().unwrap().one(), 1048970)
    }
}
