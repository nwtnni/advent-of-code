use std::str;

pub struct NotQuiteLisp(String);

impl str::FromStr for NotQuiteLisp {
    type Err = aoc::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NotQuiteLisp(s.to_owned()))
    }
}

impl aoc::Solution for NotQuiteLisp {
    fn one(&mut self) -> usize {
        unimplemented!()
    }

    fn two(&mut self) -> usize {
        unimplemented!()
    }
}
