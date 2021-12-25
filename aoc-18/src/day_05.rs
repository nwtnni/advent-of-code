use aoc::*;

#[derive(Clone, Debug)]
pub struct AlchemicalReduction(Vec<u8>);

impl Fro for AlchemicalReduction {
    fn fro(input: &str) -> Self {
        input.trim().as_bytes().to_owned().tap(Self)
    }
}

impl Solution for AlchemicalReduction {
    fn one(mut self) -> i64 {
        self.react();
        self.0.len() as i64
    }

    fn two(self) -> i64 {
        (b'A'..=b'Z')
            .map(|letter| {
                self.clone()
                    .tap_mut(|polymer| {
                        polymer
                            .0
                            .retain(|r#type| *r#type != letter && *r#type != letter | 0b10_0000)
                    })
                    .tap_mut(|polymer| polymer.react())
            })
            .map(|polymer| polymer.0.len() as i64)
            .min()
            .unwrap()
    }
}

impl AlchemicalReduction {
    fn react(&mut self) {
        let mut change = Some(0);

        while let Some(start) = change.take() {
            for i in start..self.0.len().saturating_sub(1) {
                let a = self.0[i];
                let b = self.0[i + 1];

                if a != b && (a | 0b10_0000 == b || a & !0b10_0000 == b) {
                    change = Some(i.saturating_sub(1));
                    self.0.drain(i..i + 2);
                    break;
                }
            }
        }
    }
}
