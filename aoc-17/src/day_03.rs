use std::str;

#[derive(Debug)]
pub struct SpiralMemory(i32);

impl str::FromStr for SpiralMemory {
    type Err = aoc::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i32>()
            .map(SpiralMemory)
            .map_err(aoc::Error::InvalidInt)
    }
}

impl aoc::Solution for SpiralMemory {
    fn one(mut self) -> i32 {

        // Bottom-right corner is a square
        let inner = match (1i32..)
            .step_by(2)
            .take_while(|n| n.pow(2) < self.0)
            .last()
        {
        | None => return 0,
        | Some(inner) => inner,
        };

        let outer = inner + 1;
        
        // Steps from the bottom-right corner
        let steps = self.0 - inner.pow(2);

        if steps == 0 { return inner }

        let (dx, dy) = match steps - 1 {
        | n if n >= outer * 0 && n < outer * 1 => (1, (n % outer)),
        | n if n >= outer * 1 && n < outer * 2 => (-(n % outer), inner),
        | n if n >= outer * 2 && n < outer * 3 => (-inner, inner - (n % outer) - 1), 
        | n                                    => ((n % outer) - inner + 1, -1),
        };

        let x = ((inner / 2) + dx).abs();
        let y = (dy - (inner / 2)).abs();
        
        x + y
    }

    fn two(mut self) -> i32 {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {

    use aoc::Solution;

    use super::*;
    
    #[test]
    fn test_p1_1() {
        assert_eq!(SpiralMemory(1).one(), 0);
    }

    #[test]
    fn test_p1_12() {
        assert_eq!(SpiralMemory(12).one(), 3);
    }

    #[test]
    fn test_p1_23() {
        assert_eq!(SpiralMemory(23).one(), 2);
    }

    #[test]
    fn test_p1_1024() {
        assert_eq!(SpiralMemory(1024).one(), 31);
    }
}
