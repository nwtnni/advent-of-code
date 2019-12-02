use std::str;

#[derive(Debug)]
pub struct InverseCaptcha(Vec<u8>);

impl str::FromStr for InverseCaptcha {
    type Err = aoc::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s.split("")
            .map(aoc::Digit::from_str)
            .filter_map(Result::ok)
            .map(|digit| digit as u8)
            .collect();
        Ok(InverseCaptcha(digits))
    }
}

impl aoc::Solution for InverseCaptcha {
    fn one(mut self) -> i32 {
        if !self.0.is_empty() {
            self.0.push(self.0[0]);
        }
        self.0.windows(2)
            .filter(|ns| ns[0] == ns[1])
            .map(|ns| ns[0])
            .map(|n| n as i32)
            .sum()
    }

    fn two(mut self) -> i32 {
        let full = self.0.len();
        let half = full / 2;
        self.0.iter()
            .enumerate()
            .filter(|(i, n)| **n == self.0[(i + half) % full])
            .map(|(_, n)| n)
            .map(|n| *n as i32)
            .sum()
    }
}
