use aoc::*;

const SUBJECT: i64 = 7;
const MOD: i64 = 20201227;

#[derive(Clone, Debug)]
pub struct ComboBreaker {
    card: i64,
    door: i64,
}

impl Fro for ComboBreaker {
    fn fro(input: &str) -> Self {
        let mut iter = input
            .trim()
            .split('\n')
            .map(i64::fro);

        Self {
            card: iter.give(),
            door: iter.give(),
        }
    }
}

impl Solution for ComboBreaker {
    fn one(self) -> i64 {
        let mut card = 1;
        let mut card_loop = 0;

        while card != self.card {
            card = (card * SUBJECT) % MOD;
            card_loop += 1;
        }

        let mut door = 1;
        for _ in 0..card_loop {
            door = (door * self.door) % MOD;
        }

        door
    }

    fn two(self) -> i64 {
        todo!()
    }
}
