use std::cmp;
use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct DiracDice(usize, usize);

impl Fro for DiracDice {
    fn fro(input: &str) -> Self {
        let mut lines = input.trim().split('\n');
        let (a, b) = (lines.give(), lines.give());

        let a = a
            .trim()
            .trim_start_matches("Player 1 starting position: ")
            .tap(usize::fro);
        let b = b
            .trim()
            .trim_start_matches("Player 2 starting position: ")
            .tap(usize::fro);

        Self(a, b)
    }
}

impl Solution for DiracDice {
    fn one(self) -> i64 {
        let mut dice = (1..=100).cycle().enumerate();

        let mut points = [0, 0];
        let mut places = [self.0, self.1];
        let mut turn = 0;

        while points[0] < 1000 && points[1] < 1000 {
            let roll = dice.give().1 + dice.give().1 + dice.give().1;

            let place = match (places[turn] + roll) % 10 {
                0 => 10,
                n => n,
            };

            points[turn] += place;
            places[turn] = place;
            turn ^= 1;
        }

        (points[turn] * dice.give().0) as i64
    }

    fn two(self) -> i64 {
        fn recurse(
            points: [u8; 2],
            places: [u8; 2],
            turn: bool,
            memo: &mut HashMap<([u8; 2], [u8; 2], bool), (usize, usize)>,
        ) -> (usize, usize) {
            if let Some(score) = memo.get(&(points, places, turn)) {
                return *score;
            }

            if points[0] >= 21 || points[1] >= 21 {
                let winner = if points[0] > points[1] {
                    (1, 0)
                } else {
                    (0, 1)
                };
                memo.insert((points, places, turn), winner);
                return winner;
            }

            let mut a_won = 0;
            let mut b_won = 0;

            for roll_a in 1..=3 {
                for roll_b in 1..=3 {
                    for roll_c in 1..=3 {
                        let place = match (places[turn as usize] + roll_a + roll_b + roll_c) % 10 {
                            0 => 10,
                            n => n,
                        };

                        let mut points_ = points.clone();
                        let mut places_ = places.clone();

                        points_[turn as usize] += place;
                        places_[turn as usize] = place;

                        let (a, b) = recurse(points_, places_, turn ^ true, memo);
                        a_won += a;
                        b_won += b;
                    }
                }
            }

            memo.insert((points, places, turn), (a_won, b_won));
            (a_won, b_won)
        }

        let (a, b) = recurse(
            [0, 0],
            [self.0 as u8, self.1 as u8],
            false,
            &mut HashMap::new(),
        );
        cmp::max(a, b) as i64
    }
}
