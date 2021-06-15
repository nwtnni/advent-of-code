use std::mem;

use aoc::*;

#[derive(Clone, Debug)]
pub struct ElvesLookElvesSay(Vec<u8>);

impl Fro for ElvesLookElvesSay {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .bytes()
            .map(|byte| byte - b'0')
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl ElvesLookElvesSay {
    fn expand(self, iterations: usize) -> i64 {
        let mut prev = Vec::new();
        let mut next = self.0;

        for _ in 0..iterations {
            mem::swap(&mut prev, &mut next);
            next.clear();

            let mut state = None;

            for step in &prev {
                match state {
                    None => {
                        state = Some((*step, 1));
                    }
                    Some((byte, count)) if *step == byte => {
                        state = Some((byte, count + 1));
                    }
                    Some((byte, count)) => {
                        state = Some((*step, 1));
                        next.push(count);
                        next.push(byte);
                    }
                }
            }

            if let Some((byte, count)) = state {
                next.push(count);
                next.push(byte);
            }
        }

        next.len() as i64
    }
}

impl Solution for ElvesLookElvesSay {
    fn one(self) -> i64 {
        self.expand(40)
    }

    fn two(self) -> i64 {
        self.expand(50)
    }
}
