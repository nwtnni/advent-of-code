use std::collections::HashSet;
use std::collections::VecDeque;

use aoc::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CrabCombat {
    one: VecDeque<i64>,
    two: VecDeque<i64>,
}

impl Fro for CrabCombat {
    fn fro(input: &str) -> Self {
        let mut iter = input.trim().split("\n\n");

        let one = iter
            .give()
            .trim()
            .split('\n')
            .skip(1)
            .map(i64::fro)
            .collect::<VecDeque<_>>();

        let two = iter
            .give()
            .trim()
            .split('\n')
            .skip(1)
            .map(i64::fro)
            .collect::<VecDeque<_>>();

        Self { one, two }
    }
}

impl Solution for CrabCombat {
    fn one(mut self) -> i64 {
        let total = self.one.len() + self.two.len();

        while self.one.len() < total && self.two.len() < total {
            let a = self.one.pop_front().unwrap();
            let b = self.two.pop_front().unwrap();
            if a > b {
                self.one.push_back(a);
                self.one.push_back(b);
            } else {
                self.two.push_back(b);
                self.two.push_back(a);
            }
        }

        let deck = if self.one.len() == total {
            &self.one
        } else {
            &self.two
        };

        deck.iter()
            .enumerate()
            .map(|(index, card)| (total - index) as i64 * card)
            .sum::<i64>()
    }

    fn two(mut self) -> i64 {
        let total = self.one.len() + self.two.len();
        let deck = if recurse(&mut self) {
            &self.two
        } else {
            &self.one
        };
        deck.iter()
            .enumerate()
            .map(|(index, card)| (total - index) as i64 * card)
            .sum::<i64>()
    }
}

fn recurse(game: &mut CrabCombat) -> bool {
    let total = game.one.len() + game.two.len();
    let mut seen = HashSet::new();

    while game.one.len() < total && game.two.len() < total {
        if !seen.insert(game.clone()) {
            return false;
        }

        let a = game.one.pop_front().unwrap();
        let b = game.two.pop_front().unwrap();

        let winner = if game.one.len() as i64 >= a && game.two.len() as i64 >= b {
            let mut game = game.clone();
            game.one.truncate(a as usize);
            game.two.truncate(b as usize);
            recurse(&mut game)
        } else {
            b > a
        };

        if winner {
            game.two.push_back(b);
            game.two.push_back(a);
        } else {
            game.one.push_back(a);
            game.one.push_back(b);
        }
    }

    game.two.len() == total
}
