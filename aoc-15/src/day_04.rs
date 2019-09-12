use std::fmt::Write;
use std::str;

pub struct TheIdealStockingStuffer {
    buf: String,
    key: String,
}

impl str::FromStr for TheIdealStockingStuffer {
    type Err = aoc::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            buf: String::default(),
            key: s.trim().to_owned(),
        })
    }
}

impl TheIdealStockingStuffer {
    #[inline(always)]
    fn hash(&mut self, salt: i32) -> &str {
        self.buf.clear();
        write!(self.buf, "{}{}", self.key, salt).ok();
        let digest = md5::compute(&self.buf);
        self.buf.clear();
        write!(self.buf, "{:x}", digest).ok();
        &self.buf
    }

    fn find(&mut self, len: usize) -> i32 {
        (1i32..).find(|&salt| {
            self.hash(salt)
                .as_bytes()
                .iter()
                .take(len)
                .all(|b| *b == b'0')
        }).unwrap()
    }
}

impl aoc::Solution for TheIdealStockingStuffer {
    fn one(&mut self) -> i32 {
        self.find(5)
    }

    fn two(&mut self) -> i32 {
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
