use std::fmt::Write;

use aoc::*;

#[derive(Clone, Debug)]
pub struct TheIdealStockingStuffer {
    buf: String,
    len: usize,
}

impl Fro for TheIdealStockingStuffer {
    fn fro(input: &str) -> Self {
        Self {
            buf: input.trim().to_owned(),
            len: input.trim().len(),
        }
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

impl Solution for TheIdealStockingStuffer {
    fn one(mut self) -> i64 {
        self.find(5)
    }

    fn two(mut self) -> i64 {
        self.find(6)
    }
}

#[cfg(test)]
mod tests {

    use aoc::Fro as _;
    use aoc::Solution as _;

    #[test]
    fn test_0() {
        assert_eq!(super::TheIdealStockingStuffer::fro("abcdef").one(), 609043)
    }

    #[test]
    fn test_1() {
        assert_eq!(super::TheIdealStockingStuffer::fro("pqrstuv").one(), 1048970)
    }
}
