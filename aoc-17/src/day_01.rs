use std::str;

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
    fn one(mut self) -> usize {
        if !self.0.is_empty() {
            self.0.push(self.0[0]);
        }
        self.0.windows(2)
            .filter(|digits| digits[0] == digits[1])
            .map(|digits| digits[0])
            .map(|digit| digit as usize)
            .sum()
    }

    fn two(self) -> usize {
        unimplemented!()
    }
}
