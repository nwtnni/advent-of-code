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
        let half = self.0.len() >> 1;
        let mut gamma = 0;

        for bit in 0..12 {
            let ones = self
                .0
                .iter()
                .filter(|number| *number & (1 << bit) > 0)
                .count();

            gamma |= ((ones > half) as u16) << bit;
        }

        let epsilon = gamma ^ 0b1111_1111_1111;
        gamma as i64 * epsilon as i64
    }

    fn two(self) -> i64 {
        let mut generator = self.0.clone();

        for bit in (0..12).rev() {
            let half = generator.len() >> 1;
            let even = generator.len() & 1 == 0;
            let ones = generator
                .iter()
                .filter(|number| *number & (1 << bit) > 0)
                .count();

            generator.retain(|number| {
                let tied = (even && ones == half) as u16;
                let more = (ones > half) as u16;
                *number & (1 << bit) == (tied | more) << bit
            });

            if generator.len() == 1 {
                break;
            }
        }

        let mut scrubber = self.0.clone();

        for bit in (0..12).rev() {
            let half = scrubber.len() >> 1;
            let even = scrubber.len() & 1 == 0;
            let ones = scrubber
                .iter()
                .filter(|number| *number & (1 << bit) > 0)
                .count();

            scrubber.retain(|number| {
                let tied = (even && ones == half) as u16;
                let more = (ones > half) as u16;
                *number & (1 << bit) != (tied | more) << bit
            });

            if scrubber.len() == 1 {
                break;
            }
        }

        generator[0] as i64 * scrubber[0] as i64
    }
}
