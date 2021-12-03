use aoc::*;

#[derive(Clone, Debug)]
pub struct BinaryDiagnostic(Vec<u16>);

impl Fro for BinaryDiagnostic {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| u16::from_str_radix(line, 2).unwrap())
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for BinaryDiagnostic {
    fn one(self) -> i64 {
        let mut gamma = 0;
        let mut epsilon = 0;
        let half = self.0.len() >> 1;

        for bit in 0..12 {
            let ones = self.0
                .iter()
                .filter(|n| *n & (1 << bit) > 0)
                .count();

            if ones > half {
                gamma |= 1 << bit;
            } else {
                epsilon |= 1 << bit;
            }
        }

        gamma * epsilon
    }

    fn two(self) -> i64 {
        let mut generator = self.0.clone();
        let mut scrubber = self.0.clone();

        let mut bit = 11;
        while generator.len() > 1 {
            let half = generator.len() >> 1;
            let even = generator.len() & 1 == 0;
            let ones = generator
                .iter()
                .filter(|n| *n & (1 << bit) > 0)
                .count();

            if (ones == half && even) || ones > half {
                generator.retain(|n| *n & (1 << bit) > 0);
            } else {
                generator.retain(|n| *n & (1 << bit) == 0);
            }

            bit -= 1;
        }

        let mut bit = 11;
        while scrubber.len() > 1 {
            let half = scrubber.len() >> 1;
            let even = scrubber.len() & 1 == 0;

            let ones = scrubber
                .iter()
                .filter(|n| *n & (1 << bit) > 0)
                .count();

            if (ones == half && even) || ones > half {
                scrubber.retain(|n| *n & (1 << bit) == 0);
            } else {
                scrubber.retain(|n| *n & (1 << bit) > 0);
            }

            bit -= 1;
        }

        generator[0] as i64 * scrubber[0] as i64
    }
}
