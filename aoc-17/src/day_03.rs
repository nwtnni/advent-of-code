use std::str;

#[derive(Debug)]
pub struct SpiralMemory(usize);

impl str::FromStr for SpiralMemory {
    type Err = aoc::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<usize>()
            .map(SpiralMemory)
            .map_err(aoc::Error::InvalidInt)
    }
}
