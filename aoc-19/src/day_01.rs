use std::str;

pub struct TheTyrannyOfTheRocketEquation(Vec<i32>);

impl str::FromStr for TheTyrannyOfTheRocketEquation {
    type Err = aoc::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input.split_whitespace()
            .map(|line| line.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .map(TheTyrannyOfTheRocketEquation)
            .map_err(aoc::Error::InvalidInt)
    }
}

impl aoc::Solution for TheTyrannyOfTheRocketEquation {
    fn one(&mut self) -> i32 {
        self.0.iter()
            .map(|mass| mass / 3 - 2)
            .sum()
    }

    fn two(&mut self) -> i32 {
        unimplemented!()
    }
}
