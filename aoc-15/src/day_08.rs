use std::mem;

use aoc::*;

#[derive(Clone, Debug)]
pub struct Matchsticks(Vec<String>);

impl Fro for Matchsticks {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(String::from)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for Matchsticks {
    fn one(self) -> i64 {
        let mut code = 0;
        let mut memory = 0;
        for literal in self.0 {
            code += literal.len() as i64;
            memory += literal
                .chars()
                .skip(1)
                .take(literal.len() - 2)
                .scan(false, |escape, char| match (mem::take(escape), char) {
                    (false, '\\') => {
                        *escape = true;
                        Some(1)
                    }
                    (false, _) => Some(1),
                    (true, '\\') => Some(0),
                    (true, '"') => Some(0),
                    (true, 'x') => Some(-2),
                    (true, _) => unreachable!(),
                })
                .sum::<i64>();
        }
        code - memory
    }

    fn two(self) -> i64 {
        let mut original = 0;
        let mut encoded = 0;
        for literal in self.0 {
            original += literal.len() as i64;
            encoded += literal
                .chars()
                .map(|char| match char {
                    '\\' => 2,
                    '"' => 2,
                    _ => 1,
                })
                .sum::<i64>()
                + 2;
        }
        encoded - original
    }
}
